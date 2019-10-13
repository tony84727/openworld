use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::Transform,
    ecs::{Entities, EntityBuilder, Read, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::{
        rendy::mesh::PosTex,
        shape::{Shape, ShapePrefab},
        Mesh, MeshPrefab,
    },
};
use nalgebra::Vector3;
use serde::export::PhantomData;

use crate::physics::Ground;

pub struct RandomCubeTerra {
    floor_handle: Option<Handle<Mesh>>,
    activated: bool,
}

impl RandomCubeTerra {
    pub fn new() -> Self {
        RandomCubeTerra {
            floor_handle: None,
            activated: false,
        }
    }
    fn load_floor(&mut self, loader: &Loader, storage: &AssetStorage<Mesh>) {
        if self.floor_handle.is_none() {
            self.floor_handle = Some(
                loader.load_from_data(
                    Shape::Cube
                        .generate::<Vec<PosTex>>(Some((1.0, 0.2, 1.0)))
                        .into(),
                    (),
                    storage,
                ),
            );
        }
    }
}

impl<'a> System<'a> for RandomCubeTerra {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Ground>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Handle<Mesh>>,
        ReadExpect<'a, Loader>,
        Read<'a, AssetStorage<Mesh>>,
    );

    fn run(
        &mut self,
        (entities, mut ground, mut transform, mut meshes, loader, mesh_storage): Self::SystemData,
    ) {
        if self.activated {
            return;
        }
        self.load_floor(&*loader, &*mesh_storage);
        for x in 0..100 {
            for z in 0..100 {
                let entity = entities
                    .build_entity()
                    .with(Ground { size: 1.0 }, &mut ground)
                    .with(
                        Transform::from(Vector3::from([x as f32, -10.0, z as f32])),
                        &mut transform,
                    )
                    .with(
                        self.floor_handle
                            .as_ref()
                            .expect("floor mesh not loaded yet")
                            .clone(),
                        &mut meshes,
                    )
                    .build();
            }
        }
        self.activated = true;
    }
}
