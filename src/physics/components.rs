use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::Component,
    ecs::{DenseVecStorage, Entity, NullStorage, WriteStorage},
    Error,
};
use nphysics3d::object::DefaultBodyHandle;
use serde::{Deserialize, Serialize};

#[derive(Clone, PrefabData, Serialize, Deserialize)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
// a rigidbody with a [size] sphere collider
pub struct RigidBody {
    pub size: f32,
}

impl Component for RigidBody {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, PrefabData, Serialize, Deserialize)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
// a rigidbody component with ground state(ignore forces. nearly infinite mass)
pub struct Ground {
    pub size: f32,
}

impl Component for Ground {
    type Storage = DenseVecStorage<Self>;
}

pub struct PhysicsState {
    pub(crate) body: DefaultBodyHandle,
    //    pub(crate) collider: DefaultColliderHandle,
}

impl Component for PhysicsState {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct DynamicPhysicsObject;

impl Component for DynamicPhysicsObject {
    type Storage = NullStorage<Self>;
}

// for tagging the objects the player is controlling.
#[derive(Default)]
pub struct PlayerTag;

impl Component for PlayerTag {
    type Storage = NullStorage<Self>;
}
