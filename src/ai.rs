use rand::*;

pub struct Layer {
    weights: Vec<f64>,
    biases: Vec<f64>,
}

pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetwork {
    fn init_nn() -> Self {
        let weights = vec![random(), random()];
        let biases = vec![random()];

        return Self { weights, biases };
    }
}
