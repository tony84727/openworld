use std::ops::DerefMut;

use amethyst::{
    controls::{HideCursor, WindowFocus},
    core::{
        math::Vector3,
        shrev::{EventChannel, ReaderId},
        SystemDesc, Transform,
    },
    ecs::{Entities, Join, Read, ReadStorage, System, World, Write, WriteStorage},
    input::{InputHandler, StringBindings},
};
use nalgebra::{Isometry3, UnitQuaternion};
use ncollide3d::shape::{Cuboid, ShapeHandle};
use nphysics3d::algebra::{Force3, ForceType};
use nphysics3d::object::{Body, BodyStatus, DefaultBodyHandle};
use nphysics3d::{
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    object::{BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, RigidBodyDesc},
    world::{DefaultGeometricalWorld, DefaultMechanicalWorld},
};
use winit::{DeviceEvent, Event};

use crate::physics::DynamicPhysicsObject;

use super::components::{Ground, PhysicsState, PlayerTag, RigidBody};
use amethyst::prelude::WorldExt;

/**
    PhysicsWorld resource holds states of nphysics
*/
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

/// PhysicsWorldSystem is responsible to maintain PhysicsWorld state.
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
    body_status: BodyStatus,
    no_gravity: bool,
}

impl<'a> BodyCreator<'a> {
    fn new(
        bodies: &'a mut DefaultBodySet<f64>,
        colliders: &'a mut DefaultColliderSet<f64>,
    ) -> Self {
        BodyCreator {
            bodies,
            colliders,
            body_status: BodyStatus::Dynamic,
            no_gravity: false,
        }
    }
    fn set_body_status(&mut self, status: BodyStatus) -> &mut Self {
        self.body_status = status;
        self
    }

    fn set_no_gravity(&mut self) -> &mut Self {
        self.no_gravity = true;
        self
    }

    fn create(&mut self, collider_size: f64, transform: &Transform) -> DefaultBodyHandle {
        let transform_isometry = transform.isometry();
        let body = RigidBodyDesc::new()
            .position(nalgebra::convert::<Isometry3<f32>, Isometry3<f64>>(
                *transform_isometry,
            ))
            .status(self.body_status)
            .gravity_enabled(!self.no_gravity)
            .build();
        let shape_handle = ShapeHandle::new(Cuboid::new(Vector3::from([
            collider_size / 2.,
            collider_size / 2.,
            collider_size / 2.,
        ])));
        let body_handle = self.bodies.insert(body);
        let collider = ColliderDesc::new(shape_handle)
            .density(1.0)
            .build(BodyPartHandle(body_handle, 0));
        let _collider_handle = self.colliders.insert(collider);
        body_handle
    }
}

/// NPhysicsSystem create corresponding body for a entity with RigidBody.
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
        (entities, rigidbody, ground, mut phystate, mut transform, mut world, mut dynamic): Self::SystemData,
    ) {
        {
            let world = world.deref_mut();
            let mut creator = BodyCreator::new(&mut world.bodies, &mut world.colliders);
            let mut to_insert = Vec::new();
            for (e, r, _, transform) in (&entities, &rigidbody, !&phystate, &transform).join() {
                to_insert.push((
                    e,
                    PhysicsState {
                        body: creator.create(r.size as f64, &transform),
                    },
                ));
                dynamic.insert(e, DynamicPhysicsObject).unwrap();
            }
            // create ground bodies
            creator.set_body_status(BodyStatus::Static).set_no_gravity();
            for (e, g, _, t) in (&entities, &ground, !&phystate, &transform).join() {
                to_insert.push((
                    e,
                    PhysicsState {
                        body: creator.create(g.size as f64, t),
                    },
                ));
            }
            for (e, state) in to_insert {
                phystate.insert(e, state).unwrap();
            }
        }
        for (t, state, _) in (&mut transform, &phystate, &dynamic).join() {
            if let Some(body) = world.bodies.rigid_body(state.body) {
                let iso = body.position();
                t.set_translation(iso.translation.vector);
                t.set_rotation(iso.rotation);
            }
        }
    }
}

pub struct PlayerRotateSystem {
    event_reader: ReaderId<Event>,
}

impl<'a> System<'a> for PlayerRotateSystem {
    type SystemData = (
        Read<'a, EventChannel<Event>>,
        Read<'a, WindowFocus>,
        Read<'a, HideCursor>,
        ReadStorage<'a, PlayerTag>,
        ReadStorage<'a, PhysicsState>,
        Write<'a, PhysicsWorld>,
    );

    fn run(&mut self, (events, focus, hide, tag, state, mut world): Self::SystemData) {
        for event in events.read(&mut self.event_reader) {
            if focus.is_focused && hide.hide {
                if let Event::DeviceEvent { ref event, .. } = *event {
                    if let DeviceEvent::MouseMotion { delta: (x, y) } = event {
                        for (_, state) in (&tag, &state).join() {
                            let body = world.bodies.rigid_body_mut(state.body).unwrap();
                            let mut new_position = body.position().clone();
                            new_position.append_rotation_mut(&UnitQuaternion::from_axis_angle(
                                &Vector3::x_axis(),
                                *y * 0.1,
                            ));
                            new_position.append_rotation_mut(&UnitQuaternion::from_axis_angle(
                                &Vector3::y_axis(),
                                *x * 0.1,
                            ));
                            body.set_position(new_position);
                        }
                    }
                }
            }
        }
    }
}

pub struct PlayerRotateSystemDesc;

impl SystemDesc<'_, '_, PlayerRotateSystem> for PlayerRotateSystemDesc {
    fn build(self, world: &mut World) -> PlayerRotateSystem {
        let mut channel = world.write_resource::<EventChannel<Event>>();
        let reader_id = channel.register_reader();
        PlayerRotateSystem {
            event_reader: reader_id,
        }
    }
}

pub struct PlayerInputSystem;

impl<'a> System<'a> for PlayerInputSystem {
    type SystemData = (
        ReadStorage<'a, PlayerTag>,
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
                    body.set_linear_damping(0.5);
                    body.set_angular_damping(0.5);
                    body.apply_force(0, &force, ForceType::VelocityChange, true);
                }
            }
        }
    }
}
