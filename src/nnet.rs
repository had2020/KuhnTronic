pub fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + -x.exp());
}

pub fn dSigmoid(x: f64) -> f64 {
    return x * (1.0 - x); // Derivative
}

/*// TODO SIMD registers for more compution.
pub fn sigmoid_int(x: i64) -> i64 {
    return x; // TODO
}*/
