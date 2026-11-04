use rand::*;

pub struct Layer {
    pub weights: Vec<f64>,
    pub biases: Vec<f64>,
}

pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
}

impl Layer {
    pub fn init_layer() -> Self {
        return Layer {
            weights: vec![random(), random(), random(), random(), random(), random()],
            biases: vec![random(), random()],
        };
    }
}
