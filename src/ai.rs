use rand::*;

pub struct Layer {
    pub biases: Vec<f64>,
}

pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
    pub weights: Vec<f64>,
}

impl NeuralNetwork {
    pub fn init_nn() -> Self {
        let mut layers: Vec<Layer> = Vec::new();
        layers.push(Layer { biases: vec![0.0] });
        layers.push(Layer {
            biases: vec![random(), random()],
        });
        layers.push(Layer { biases: vec![0.0] });
        return NeuralNetwork {
            layers,
            weights: vec![random(), random(), random(), random(), random(), random()],
        };
    }
    pub fn run_through_net(&self, inputs: &[f64; 2]) -> f64 {
        let mut total_sum: f64 = 0.0;
        for layer in self.layers.iter() {
            for bias in layer.biases.iter() {
                total_sum += bias;
            }
        }
        println!("total nn: {}", total_sum);
        return total_sum;
    }
}
