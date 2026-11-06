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
            biases: vec![random_range(-1.0..1.0), random_range(-1.0..1.0)],
        });
        layers.push(Layer { biases: vec![0.0] });
        return NeuralNetwork {
            layers,
            weights: vec![
                random_range(-1.0..1.0),
                random_range(-1.0..1.0),
                random_range(-1.0..1.0),
                random_range(-1.0..1.0),
                random_range(-1.0..1.0),
                random_range(-1.0..1.0),
            ],
        };
    }

    pub fn run_through_net(&self, inputs: &[f64; 2]) -> f64 {
        let mut total_sum: f64 = 0.0;
        for layer in self.layers.iter() {
            for bias in layer.biases.iter() {
                for (index, weight) in self.weights.iter().enumerate() {
                    if index < 2 {
                        total_sum += inputs[index] * weight + bias;
                    }
                }
            }
        }
        // ReLU check (set every negative number to 0)
        if total_sum < 0.0 {
            total_sum = 0.0;
        }
        println!("total nn: {}", total_sum);
        return total_sum;
    }
}
