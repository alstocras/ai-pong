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
        layers.push(Layer {
            biases: vec![0.0, 0.0, 0.0, 0.0],
        });
        layers.push(Layer {
            biases: vec![
                random_range(-1.0..1.0),
                random_range(-1.0..1.0),
                random_range(-1.0..1.0),
            ],
        });
        layers.push(Layer {
            biases: vec![
                random_range(-1.0..1.0),
                random_range(-1.0..1.0),
                random_range(-1.0..1.0),
            ],
        });
        layers.push(Layer {
            biases: vec![0.0, 0.0],
        });
        let target_weight_num: u64 = 27;
        let mut weights: Vec<f64> = Vec::new();

        for _ in 0..target_weight_num {
            weights.push(random_range(-1.0..1.0));
        }

        return NeuralNetwork { layers, weights };
    }

    pub fn run_through_net(&self, inputs: &[f64; 4]) -> f64 {
        let mut total_sum: f64 = 0.0;
        for layer in &self.layers {
            for bias in &layer.biases {
                total_sum += bias;
            }
            for (index, weight) in self.weights.iter().enumerate() {
                for (input_index, input) in inputs.iter().enumerate() {
                    total_sum += input * weight
                }
            }
            // ReLU on every layer
            if total_sum < 0.0 {
                total_sum = 0.0;
            }
        }

        // final sigmoid at the end
        total_sum = 1.0 / (1.0 + (-total_sum).exp());

        println!("total nn: {}", total_sum);
        return total_sum;
    }
}
