use std::ops::{Deref, DerefMut};

use amethyst::{
    core::{math::Vector3, Transform},
    ecs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};
use nalgebra::Isometry3;
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::algebra::{Force3, ForceType};
use nphysics3d::object::{Body, BodyStatus, DefaultBodyHandle};
use nphysics3d::{
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    object::{BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, RigidBodyDesc},
    world::{DefaultGeometricalWorld, DefaultMechanicalWorld},
};

use crate::physics::DynamicPhysicsObject;

use super::components::{ForceTag, Ground, PhysicsState, RigidBody};

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

struct BodyCreator<'a> {
    bodies: &'a mut DefaultBodySet<f64>,
    colliders: &'a mut DefaultColliderSet<f64>,
    collider_size: f64,
    body_status: BodyStatus,
}

impl<'a> BodyCreator<'a> {
    fn new(
        bodies: &'a mut DefaultBodySet<f64>,
        colliders: &'a mut DefaultColliderSet<f64>,
    ) -> Self {
        BodyCreator {
            bodies,
            colliders,
            collider_size: 1.0,
            body_status: BodyStatus::Dynamic,
        }
    }
    fn set_body_status(&mut self, status: BodyStatus) -> &mut Self {
        self.body_status = status;
        self
    }

    fn create(&mut self, collider_size: f64, transform: &Transform) -> DefaultBodyHandle {
        let transform_isometry = transform.isometry();
        let body = RigidBodyDesc::new()
            .position(nalgebra::convert::<Isometry3<f32>, Isometry3<f64>>(
                *transform_isometry,
            ))
            .status(self.body_status)
            .build();
        let shape_handle = ShapeHandle::new(Cuboid::new(Vector3::from([
            collider_size,
            collider_size,
            collider_size,
        ])));
        let body_handle = self.bodies.insert(body);
        let collider = ColliderDesc::new(shape_handle)
            .density(1.0)
            .build(BodyPartHandle(body_handle, 0));
        let _collider_handle = self.colliders.insert(collider);
        body_handle
    }
}

pub struct NPhysicsSystem;

impl<'a> System<'a> for NPhysicsSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, RigidBody>,
        ReadStorage<'a, Ground>,
        WriteStorage<'a, PhysicsState>,
        WriteStorage<'a, Transform>,
        Write<'a, PhysicsWorld>,
        WriteStorage<'a, DynamicPhysicsObject>,
    );

    fn run(
        &mut self,
        (entities, rigidbody, ground, mut rigidstate, mut transform, mut world, mut dynamic): Self::SystemData,
    ) {
        {
            let mut world = world.deref_mut();
            let mut creator = BodyCreator::new(&mut world.bodies, &mut world.colliders);
            let mut to_insert = Vec::new();
            for (e, r, _, transform) in (&entities, &rigidbody, !&rigidstate, &transform).join() {
                to_insert.push((
                    e,
                    PhysicsState {
                        body: creator.create(r.size as f64, &transform),
                    },
                ));
                dynamic.insert(e, DynamicPhysicsObject);
            }
            // create ground bodies
            creator.set_body_status(BodyStatus::Static);
            for (e, g, _, t) in (&entities, &ground, !&rigidstate, &transform).join() {
                to_insert.push((
                    e,
                    PhysicsState {
                        body: creator.create(g.size as f64, t),
                    },
                ));
            }
            for (e, state) in to_insert {
                rigidstate.insert(e, state).unwrap();
            }
        }
        for (t, state, _) in (&mut transform, &rigidstate, &dynamic).join() {
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
        ReadStorage<'a, PhysicsState>,
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
