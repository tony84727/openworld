use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::Transform,
    derive::PrefabData,
    ecs::Entity,
    renderer::{
        formats::GraphicsPrefab,
        rendy::mesh::{Normal, Position, TexCoord},
    },
    Error,
};
use serde::{Deserialize, Serialize};

use crate::physics::RigidBody;

#[derive(PrefabData, Serialize, Deserialize)]
pub struct ScenePrefabData {
    graphics: Option<GraphicsPrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>>,
    transform: Option<Transform>,
    cube: Option<RigidBody>,
}
