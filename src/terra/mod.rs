use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::Transform,
    ecs::{Entities, EntityBuilder, Read, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::{
        formats::{mtl::MaterialPrefab, GraphicsPrefab},
        rendy::mesh::PosTex,
        shape::{Shape, ShapePrefab},
        Material, Mesh, MeshPrefab,
    },
};
use nalgebra::Vector3;
use serde::export::PhantomData;

use crate::physics::Ground;

pub struct RandomCubeTerra {
    floor_prefab: GraphicsPrefab<Vec<PosTex>>,
    activated: bool,
}

impl RandomCubeTerra {
    pub fn new() -> Self {
        RandomCubeTerra {
            activated: false,
            floor_prefab: GraphicsPrefab {
                material: MaterialPrefab::default(),
                mesh: MeshPrefab::Shape(ShapePrefab {
                    handle: None,
                    shape_scale: Some((1.0, 0.5, 1.0)),
                    _m: PhantomData::default(),
                    shape: Shape::Cube,
                }),
            },
        }
    }
}

impl<'a> System<'a> for RandomCubeTerra {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Ground>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut ground,
            mut transform,
            mut meshes,
            mut materials,
            loader,
            mesh_storage,
            material_storage,
        ): Self::SystemData,
    ) {
        if self.activated {
            return;
        }
        self.load_floor(&*loader, &*mesh_storage, &*material_storage);
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
