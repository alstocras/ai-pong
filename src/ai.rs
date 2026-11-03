pub struct Neuron {
    layer: Layer,
    bias: f32,
}

pub enum Layer {
    Output,
    Input,
    Hidden,
}
