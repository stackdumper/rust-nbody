use cgmath::Vector2;
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Velocity {
    pub data: Vector2<f64>,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
