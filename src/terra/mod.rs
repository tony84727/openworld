use amethyst::prelude::{SystemDesc, World};
use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, RonFormat},
    core::Transform,
    ecs::{Entities, System, WriteStorage},
};
use nalgebra::Vector3;
use noise::{NoiseFn, Perlin};

use crate::physics::Ground;
use crate::prefab::ScenePrefabData;

pub trait HeightGenerator<G, H> {
    fn get_height(&self, x: G, y: G) -> H;
}

pub struct PerlinHeightGenerator {
    perlin: Perlin,
}

impl PerlinHeightGenerator {
    pub fn new() -> Self {
        PerlinHeightGenerator {
            perlin: Perlin::new(),
        }
    }
}

impl HeightGenerator<i32, f32> for PerlinHeightGenerator {
    fn get_height(&self, x: i32, y: i32) -> f32 {
        self.perlin.get([x as f64, y as f64]) as f32
    }
}

pub struct FlatHeightGenerator {
    height: f32,
}

impl FlatHeightGenerator {
    pub fn new(height: f32) -> Self {
        FlatHeightGenerator { height }
    }
}

impl HeightGenerator<i32, f32> for FlatHeightGenerator {
    fn get_height(&self, _x: i32, _y: i32) -> f32 {
        self.height
    }
}

pub struct CubeTerraSystem<G: HeightGenerator<i32, f32>> {
    height_generator: G,
    floor_prefab: Handle<Prefab<ScenePrefabData>>,
    activated: bool,
}

impl<G: HeightGenerator<i32, f32>> CubeTerraSystem<G> {
    fn new(height_generator: G, floor_prefab: Handle<Prefab<ScenePrefabData>>) -> Self {
        CubeTerraSystem {
            height_generator,
            floor_prefab,
            activated: false,
        }
    }
}

impl<'a, G: HeightGenerator<i32, f32>> System<'a> for CubeTerraSystem<G> {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Ground>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Handle<Prefab<ScenePrefabData>>>,
    );

    fn run(&mut self, (entities, mut ground, mut transform, mut prefab_store): Self::SystemData) {
        if self.activated {
            return;
        }
        for x in -50..50 {
            for z in -50..50 {
                let mut t = Transform::default();
                let height = self.height_generator.get_height(x, z) * 10.;
                t.set_translation_xyz((x * 10) as f32, height / 2., (z * 10) as f32);
                t.set_scale(Vector3::new(10., height, 10.));
                entities
                    .build_entity()
                    .with(Ground { size: 1.0 }, &mut ground)
                    .with(self.floor_prefab.clone(), &mut prefab_store)
                    .with(t, &mut transform)
                    .build();
            }
        }
        self.activated = true;
    }
}

pub struct CubeTerraSystemDesc<G: HeightGenerator<i32, f32>> {
    height_generator: G,
}

impl<G: HeightGenerator<i32, f32>> CubeTerraSystemDesc<G> {
    pub fn new(height_generator: G) -> Self {
        CubeTerraSystemDesc { height_generator }
    }
}

impl<G: HeightGenerator<i32, f32>> SystemDesc<'_, '_, CubeTerraSystem<G>>
    for CubeTerraSystemDesc<G>
{
    fn build(self, world: &mut World) -> CubeTerraSystem<G> {
        let handle = world.exec(|loader: PrefabLoader<ScenePrefabData>| {
            loader.load("prefab/floor.ron", RonFormat, ())
        });
        CubeTerraSystem::new(self.height_generator, handle)
    }
}
