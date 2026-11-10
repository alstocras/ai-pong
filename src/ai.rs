use crate::types::*;

pub struct Neuron {
    in_weight: Option<f64>,
    out_weight: Option<f64>,
    activated: bool,
    activation_func: fn(),
    value: f64,
    neuron_type: NeuronType,
    bias: f64,
}

pub struct Layer {
    neurons: Vec<Neuron>,
}
