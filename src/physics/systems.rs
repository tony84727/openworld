use amethyst::{
    core::{math::Vector3, Transform},
    ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};
use nalgebra::Isometry3;
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::algebra::{Force3, ForceType};
use nphysics3d::object::Body;
use nphysics3d::{
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    object::{BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, RigidBodyDesc},
    world::{DefaultGeometricalWorld, DefaultMechanicalWorld},
};

use super::components::{ForceTag, RigidBody, RigidBodyState};

pub struct PhysicsWorld {
    mechanical: DefaultMechanicalWorld<f64>,
    geometrical: DefaultGeometricalWorld<f64>,
    bodies: DefaultBodySet<f64>,
    colliders: DefaultColliderSet<f64>,
    joint_constraints: DefaultJointConstraintSet<f64>,
    force_generators: DefaultForceGeneratorSet<f64>,
}

impl PhysicsWorld {
    fn step(&mut self) {
        self.mechanical.step(
            &mut self.geometrical,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators,
        )
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        PhysicsWorld {
            mechanical: DefaultMechanicalWorld::new(Vector3::new(0.0, -9.8, 0.0)),
            geometrical: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators: DefaultForceGeneratorSet::new(),
        }
    }
}

pub struct PhysicsWorldSystem;

impl<'a> System<'a> for PhysicsWorldSystem {
    type SystemData = Write<'a, PhysicsWorld>;

    fn run(&mut self, mut world: Self::SystemData) {
        world.step();
    }
}

pub struct NPhysicsSystem;

impl<'a> System<'a> for NPhysicsSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, RigidBody>,
        WriteStorage<'a, RigidBodyState>,
        WriteStorage<'a, Transform>,
        Write<'a, PhysicsWorld>,
    );

    fn run(
        &mut self,
        (entities, rigidbody, mut rigidstate, mut transform, mut world): Self::SystemData,
    ) {
        {
            let mut to_insert = Vec::new();
            for (e, r, _, transform) in (&entities, &rigidbody, !&rigidstate, &transform).join() {
                let transform_isometry = transform.isometry();
                let body = RigidBodyDesc::new()
                    .position(nalgebra::convert::<Isometry3<f32>, Isometry3<f64>>(
                        *transform_isometry,
                    ))
                    .build();
                let size = r.size as f64;
                let shape_handle = ShapeHandle::new(Cuboid::new(Vector3::from([size, size, size])));
                let body_handle = world.bodies.insert(body);
                let collider = ColliderDesc::new(shape_handle)
                    .density(1.0)
                    .build(BodyPartHandle(body_handle, 0));
                let _collider_handle = world.colliders.insert(collider);
                to_insert.push((
                    e,
                    RigidBodyState {
                        body: body_handle,
                        //                        collider: collider_handle,
                    },
                ));
            }
            for (e, state) in to_insert {
                rigidstate.insert(e, state).unwrap();
            }
        }
        for (t, state) in (&mut transform, &rigidstate).join() {
            if let Some(body) = world.bodies.rigid_body(state.body) {
                let iso = body.position();
                t.set_translation(iso.translation.vector);
                t.set_rotation(iso.rotation);
            }
        }
    }
}

pub struct PlayerForceSystem;

impl<'a> System<'a> for PlayerForceSystem {
    type SystemData = (
        ReadStorage<'a, ForceTag>,
        ReadStorage<'a, RigidBodyState>,
        Write<'a, PhysicsWorld>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (tagged, state, mut world, handler): Self::SystemData) {
        let force;
        {
            if let Some(x_axis) = handler.axis_value("horizontal") {
                if let Some(y_axis) = handler.axis_value("vertical") {
                    force = Some(Force3::new(
                        Vector3::from([x_axis as f64, 0.0, y_axis as f64]),
                        Vector3::from([0.0, 0.0, 0.0]),
                    ))
                } else {
                    force = None;
                }
            } else {
                force = None;
            }
        }

        if let Some(force) = force {
            for (_, s) in (&tagged, &state).join() {
                if let Some(body) = world.bodies.rigid_body_mut(s.body) {
                    body.apply_force(0, &force, ForceType::VelocityChange, false)
                }
            }
        }
    }
}
