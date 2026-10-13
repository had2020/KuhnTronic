use rand::random;

pub const NUM_INPUTS: usize = 6;
pub const NUM_HIDDEN_NODES: usize = 16;
pub const NUM_OUTPUTS: usize = 1;

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

    pub fn train(&mut self, dataset: DataSet, epochs: usize) {
        let rng_decri: usize = dataset.inputs.len();

        for epoch in 0..epochs {}
    }
}
