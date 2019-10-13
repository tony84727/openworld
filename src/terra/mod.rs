use amethyst::prelude::{SystemDesc, World};
use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, RonFormat},
    core::Transform,
    ecs::{Entities, System, WriteStorage},
};
use noise::{NoiseFn, Perlin};

use crate::physics::Ground;
use crate::prefab::ScenePrefabData;

pub struct RandomCubeTerra {
    activated: bool,
    floor_prefab: Handle<Prefab<ScenePrefabData>>,
}

impl RandomCubeTerra {
    pub fn new(floor_prefab: Handle<Prefab<ScenePrefabData>>) -> Self {
        RandomCubeTerra {
            activated: false,
            floor_prefab,
        }
    }
}

pub struct RandomCubeTerraDesc;

impl SystemDesc<'_, '_, RandomCubeTerra> for RandomCubeTerraDesc {
    fn build(self, world: &mut World) -> RandomCubeTerra {
        let handle = world.exec(|loader: PrefabLoader<ScenePrefabData>| {
            loader.load("prefab/floor.ron", RonFormat, ())
        });
        RandomCubeTerra::new(handle)
    }
}

impl<'a> System<'a> for RandomCubeTerra {
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
        let perlin = Perlin::new();
        for x in -50..50 {
            for z in -50..50 {
                let mut t = Transform::default();
                t.set_translation_xyz(
                    x as f32,
                    -1.0 + perlin.get([x as f64, z as f64]) as f32,
                    z as f32,
                );
                entities
                    .build_entity()
                    .with(Ground { size: 1.0 }, &mut ground)
                    .with(t, &mut transform)
                    .with(self.floor_prefab.clone(), &mut prefab_store)
                    .build();
            }
        }
        self.activated = true;
    }
}
