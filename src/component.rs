use amethyst::ecs::{Component, DenseVecStorage};

pub struct Player {
    yaw: f32,
    pitch: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
