use macroquad::prelude::*;

pub enum GameState {
    Menu,
    Game,
    Training,
}

pub enum NeuronType {
    Hidden,
    Input,
    Output,
}

pub struct Paddle {
    pub y: f32,
    pub x: f32,
    pub speed: f32,
    pub colour: Color,
    pub length: f32,
    pub width: f32,
    pub direction: Vec2,
    pub points: u32,
}

pub struct Ball {
    pub position: Vec2,
    pub speed: f32,
    pub direction: Vec2,
    pub size: f32,
    pub colour: Color,
}
