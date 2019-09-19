use noise::{Perlin};
use amethyst::{
    ecs::{System},
}



pub struct Generator {
    perlin: Perlin,
};

impl Generator {
    pub new() -> Generator {
        Generator{
            perlin: Perlin::new(),
        }
    }
    pub fn get_y(&self, x: f64, z: f64) -> f64 {
        return perlin.get([x,z])
    }
}

pub struct TerraSystem {
    border: Point<f64>
}

impl System<'_> for TerraSystem {
}