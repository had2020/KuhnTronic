use rand::{random, random_range};
use serde::{Deserialize, Serialize};

pub const NUM_INPUTS: usize = 6;
pub const NUM_HIDDEN_NODES: usize = 16;
pub const NUM_OUTPUTS: usize = 1;

pub const LEARNING_RATE: f64 = 0.1;

pub fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + -x.exp());
}

pub fn d_sigmoid(x: f64) -> f64 {
    return x * (1.0 - x); // Derivative
}

/*// TODO SIMD registers for more compution.
pub fn sigmoid_int(x: i64) -> i64 {
    return x; // TODO
}*/

pub struct DataSet {
    pub inputs: Vec<[f64; NUM_INPUTS]>,
    pub outputs: Vec<[f64; NUM_OUTPUTS]>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Model {
    pub hidden_layer: [f64; NUM_HIDDEN_NODES],
    pub output_layer: [f64; NUM_OUTPUTS],

    pub hidden_layer_bias: [f64; NUM_HIDDEN_NODES],
    pub output_layer_bias: [f64; NUM_OUTPUTS],

    pub hidden_weights: [[f64; NUM_OUTPUTS]; NUM_HIDDEN_NODES],
    pub output_weights: [[f64; NUM_OUTPUTS]; NUM_OUTPUTS],
}

impl Model {
    pub fn new() -> Self {
        let mut r: Model = Model {
            hidden_layer: [0.0; NUM_HIDDEN_NODES],
            output_layer: [0.0; NUM_OUTPUTS],
            hidden_layer_bias: [0.0; NUM_HIDDEN_NODES],
            output_layer_bias: [0.0; NUM_OUTPUTS],
            hidden_weights: [[0.0; NUM_OUTPUTS]; NUM_HIDDEN_NODES],
            output_weights: [[0.0; NUM_OUTPUTS]; NUM_OUTPUTS],
        };

        // TODO: optimize
        for i in 0..r.hidden_layer.len() {
            r.hidden_layer[i] = random();
        }
        for i in 0..r.output_layer.len() {
            r.output_layer[i] = random();
        }
        for i in 0..r.hidden_layer_bias.len() {
            r.hidden_layer_bias[i] = random();
        }
        for i in 0..r.output_layer_bias.len() {
            r.output_layer_bias[i] = random();
        }
        for i in 0..r.hidden_weights.len() {
            r.hidden_weights[i] = random();
        }
        for i in 0..r.output_weights.len() {
            r.output_weights[i] = random();
        }

        return r;
    }

    pub fn train(mut self, dataset: DataSet, epochs: usize) {
        // TODO? possibley random cases from dataset?
        for _ in 0..epochs {
            let rng_set: usize = random_range(0..dataset.inputs.len());
            std::mem::take(&mut self).forward_pass(dataset.inputs[rng_set]);
            std::mem::take(&mut self).back_prop(dataset.outputs[rng_set], dataset.inputs[rng_set]);
        }
    }

    // TODO: possible make WGPU shader
    /// Merely computes the activation functions for each layer, passing
    /// down the weights. Output is not returned, as it can be
    /// taken from the model.
    pub fn forward_pass(mut self, input: [f64; NUM_INPUTS]) {
        for i in 0..NUM_HIDDEN_NODES {
            let mut activation = self.hidden_layer_bias[i];

            for j in 0..NUM_INPUTS {
                activation += input[j] * self.hidden_weights[i][j]; // TODO: SIMD
            }

            self.hidden_layer[i] = sigmoid(activation);
        }

        for i in 0..NUM_OUTPUTS {
            let mut activation = self.output_layer_bias[i];

            for j in 0..NUM_HIDDEN_NODES {
                activation += self.hidden_layer[j] * self.output_weights[j][i];
            }
            self.output_layer[i] = sigmoid(activation);
        }
    }

    /// Merely computes chage in output weights, applys the change.
    pub fn back_prop(mut self, target_output: [f64; NUM_OUTPUTS], input: [f64; NUM_INPUTS]) {
        // Computes changes in weights
        let mut delta_output = [0.0f64; NUM_OUTPUTS];
        for i in 0..NUM_OUTPUTS {
            let output: f64 = self.output_layer[i];
            let error: f64 = target_output[i] - output;
            delta_output[i] = error * d_sigmoid(output);
        }

        let mut delta_hidden = [0.0f64; NUM_HIDDEN_NODES];
        for i in 0..NUM_HIDDEN_NODES {
            let mut error = 0.0f64;
            for j in 0..NUM_OUTPUTS {
                error += delta_output[j] * self.output_weights[i][j];
            }
            delta_hidden[i] = error * d_sigmoid(self.hidden_layer[i]);
        }

        // Applys changes in weights
        for i in 0..NUM_OUTPUTS {
            self.output_layer_bias[i] += delta_output[i] * LEARNING_RATE;
            for j in 0..NUM_HIDDEN_NODES {
                self.output_weights[j][i] += self.hidden_layer[j] * delta_output[i] * LEARNING_RATE;
            }
        }

        for i in 0..NUM_HIDDEN_NODES {
            self.hidden_layer_bias[i] += delta_hidden[i] * LEARNING_RATE;
            for j in 0..NUM_INPUTS {
                self.hidden_weights[j][i] += input[j] * delta_hidden[i] * LEARNING_RATE;
            }
        }
    }

    pub fn save_state_binary(self, filename: &str) {
        let encoded = bincode::serialize(&(dish, settings)).unwrap();
        std::fs::write(filename, encoded).unwrap();
    }
}
