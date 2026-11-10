use crate::types::*;
use rand::*;

pub struct Neuron {
    pub in_weight: Option<f64>,
    pub out_weight: Option<f64>,
    pub value: f64,
    pub neuron_type: NeuronType,
    pub bias: f64,
}

pub struct Layer {
    pub neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(neuron_amount: u64) -> Self {
        let neurons: Vec<Neuron> = Vec::new();
        for _ in 0..neuron_amount {
            neurons.push(Neuron {
                in_weight: Some(random_range(-1.0..1.0)),
                out_weight: Some(random_range(-1.0..1.0)),
                value: 0.0,
                neuron_type: NeuronType[random_range(0..2)],
            })
        }

        return Layer { neurons };
    }
}
