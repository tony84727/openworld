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

pub struct RigidBodyState {
    pub(crate) body: DefaultBodyHandle,
    //    pub(crate) collider: DefaultColliderHandle,
}

impl Component for RigidBodyState {
    type Storage = DenseVecStorage<Self>;
}

// for tagging the objects the player is controlling.
#[derive(Default)]
pub struct ForceTag;

impl Component for ForceTag {
    type Storage = NullStorage<Self>;
}
