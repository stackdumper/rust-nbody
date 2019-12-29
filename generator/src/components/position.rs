use cgmath::Vector2;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Position {
    pub data: Vector2<f64>,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}
