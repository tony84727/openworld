use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::Transform,
    ecs::{Entities, ReadExpect, ReadStorage, System, WriteStorage},
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
    mesh_prefab: MeshPrefab<Vec<PosTex>>,
    material_prefab: MaterialPrefab,
    activated: bool,
}

impl RandomCubeTerra {
    pub fn new() -> Self {
        RandomCubeTerra {
            activated: false,
            mesh_prefab: MeshPrefab::Shape(ShapePrefab {
                handle: None,
                shape_scale: Some((1.0, 0.5, 1.0)),
                _m: PhantomData::default(),
                shape: Shape::Cube,
            }),
            material_prefab: Default::default(),
        }
    }
}

impl<'a> System<'a> for RandomCubeTerra {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Ground>,
        WriteStorage<'a, Transform>,
        <MaterialPrefab as PrefabData<'a>>::SystemData,
        <MeshPrefab<Vec<PosTex>> as PrefabData<'a>>::SystemData,
    );

    fn run(
        &mut self,
        (
            entities,
            mut ground,
            mut transform,
            mut material_data,
            mut mesh_data,
        ): Self::SystemData,
    ) {
        if self.activated {
            return;
        }
        self.material_prefab.load_sub_assets(&mut ProgressCounter::new(), &mut material_data);
        self.mesh_prefab.load_sub_assets(&mut ProgressCounter::new(),&mut mesh_data);
        for x in 0..100 {
            for z in 0..100 {
                let entity = entities
                    .build_entity()
                    .with(Ground { size: 1.0 }, &mut ground)
                    .with(
                        Transform::from(Vector3::from([x as f32, -10.0, z as f32])),
                        &mut transform,
                    )
                    .build();
                self.material_prefab.add_to_entity(entity, &mut material_data, &[], &[]);
                self.mesh_prefab.add_to_entity(entity, &mut mesh_data, &[], &[]);
            }
        }
        self.activated = true;
    }
}
