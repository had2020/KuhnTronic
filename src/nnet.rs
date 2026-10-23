use rand::{random, random_range};
use serde::{Deserialize, Serialize};

/* Model layout:
First 3 inputs classify the AI's card:
    [1.0, 0.0, 0.0] -> King, [0.0, 1.0, 0.0] -> Queen, [0.0, 0.0, 1.0] -> joker.
Last 2 inputs classify the last action sequence in the game:
    Action types: {Bet: 1.0, Pass: -1.0, NULL: 0.0}
*/
pub const NUM_INPUTS: usize = 5;
pub const NUM_HIDDEN_NODES: usize = 5;
pub const NUM_OUTPUTS: usize = 1;

pub const LEARNING_RATE: f64 = 0.1;

pub fn leaky_relu(x: f64) -> f64 {
    if x > 0.0 { x } else { 0.01 * x }
}

pub fn d_leaky_relu(output: f64) -> f64 {
    if output > 0.0 { 1.0 } else { 0.01 }
}

pub struct DataSet {
    pub inputs: Vec<[f64; NUM_INPUTS]>,
    pub outputs: Vec<[f64; NUM_OUTPUTS]>,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub struct Model {
    pub hidden_layer: [f64; NUM_HIDDEN_NODES],
    pub output_layer: [f64; NUM_OUTPUTS],

    pub hidden_layer_bias: [f64; NUM_HIDDEN_NODES],
    pub output_layer_bias: [f64; NUM_OUTPUTS],

    pub hidden_weights: [[f64; NUM_INPUTS]; NUM_HIDDEN_NODES],
    pub output_weights: [[f64; NUM_HIDDEN_NODES]; NUM_OUTPUTS],
}

impl Model {
    pub fn new() -> Self {
        let mut r: Model = Model {
            hidden_layer: [0.0; NUM_HIDDEN_NODES],
            output_layer: [0.0; NUM_OUTPUTS],
            hidden_layer_bias: [0.0; NUM_HIDDEN_NODES],
            output_layer_bias: [0.0; NUM_OUTPUTS],
            hidden_weights: [[0.0; NUM_INPUTS]; NUM_HIDDEN_NODES],
            output_weights: [[0.0; NUM_HIDDEN_NODES]; NUM_OUTPUTS],
        };

        // TODO: optimize
        for i in 0..r.hidden_layer.len() {
            r.hidden_layer[i] = random_range(-0.5..0.5);
        }
        for i in 0..r.output_layer.len() {
            r.output_layer[i] = random_range(-0.5..0.5);
        }
        for i in 0..r.hidden_layer_bias.len() {
            r.hidden_layer_bias[i] = random_range(-0.5..0.5);
        }
        for i in 0..r.output_layer_bias.len() {
            r.output_layer_bias[i] = random_range(-0.5..0.5);
        }
        for i in 0..r.hidden_weights.len() {
            for j in 0..NUM_HIDDEN_NODES {
                r.hidden_weights[i][j] = random_range(-0.5..0.5);
            }
        }
        for i in 0..r.output_weights.len() {
            for j in 0..NUM_OUTPUTS {
                r.output_weights[i][j] = random_range(-0.5..0.5);
            }
        }

        return r;
    }

    pub fn train(&mut self, dataset: DataSet, epochs: usize) {
        for _ in 0..epochs {
            let rng_set: usize = random_range(0..dataset.inputs.len());
            //std::mem::
            self.forward_pass(dataset.inputs[rng_set]);
            self.back_prop(dataset.outputs[rng_set], dataset.inputs[rng_set]);
        }
    }

    // TODO: possible make WGPU shader
    /// Merely computes the activation functions for each layer, passing
    /// down the weights. Output is not returned, as it can be
    /// taken from the model.
    pub fn forward_pass(&mut self, input: [f64; NUM_INPUTS]) {
        for i in 0..NUM_HIDDEN_NODES {
            let mut activation = self.hidden_layer_bias[i];

            for j in 0..NUM_INPUTS {
                activation += input[j] * self.hidden_weights[i][j]; // TODO: SIMD
            }

            self.hidden_layer[i] = leaky_relu(activation);
        }

        for i in 0..NUM_OUTPUTS {
            let mut activation = self.output_layer_bias[i];
            for j in 0..NUM_HIDDEN_NODES {
                activation += self.hidden_layer[j] * self.output_weights[i][j];
            }
            self.output_layer[i] = leaky_relu(activation);
        }
    }

    /// Merely computes chage in output weights, applys the change.
    pub fn back_prop(&mut self, target_output: [f64; NUM_OUTPUTS], input: [f64; NUM_INPUTS]) {
        // Computes changes in weights
        let mut delta_output = [0.0f64; NUM_OUTPUTS];
        for i in 0..NUM_OUTPUTS {
            let output: f64 = self.output_layer[i];
            let error: f64 = target_output[i] - output;
            delta_output[i] = error * d_leaky_relu(output);
        }

        let mut delta_hidden = [0.0f64; NUM_HIDDEN_NODES];
        for i in 0..NUM_HIDDEN_NODES {
            let mut error = 0.0f64;
            for j in 0..NUM_OUTPUTS {
                error += delta_output[j] * self.output_weights[j][i];
            }
            delta_hidden[i] = error * d_leaky_relu(self.hidden_layer[i]);
        }

        // Applys changes in weights
        for i in 0..NUM_OUTPUTS {
            self.output_layer_bias[i] += delta_output[i] * LEARNING_RATE;
            for j in 0..NUM_HIDDEN_NODES {
                self.output_weights[i][j] += self.hidden_layer[j] * delta_output[i] * LEARNING_RATE;
            }
        }

        for i in 0..NUM_HIDDEN_NODES {
            self.hidden_layer_bias[i] += delta_hidden[i] * LEARNING_RATE;
            for j in 0..NUM_INPUTS {
                self.hidden_weights[i][j] += input[j] * delta_hidden[i] * LEARNING_RATE;
            }
        }
    }

    pub fn save_model_json(self, filename: &str) {
        let json =
            serde_json::to_string(&self).expect("\nFailed to parse model data to json format!\n");
        std::fs::write(filename, json).expect("\nFailed to write model json file to disk!\n");
    }

    pub fn load_model_json(mut self, filename: &str) {
        let content =
            std::fs::read_to_string(filename).expect("\nFailed to read model file to string!\n");
        self = serde_json::from_str(&content)
            .expect("\nFailed to assign model holder struct from parsed model data!\n");
    }
}
