use amethyst::ecs::{Entities, ReadStorage, System};

use crate::physics::ForceTag;

mod systems;

struct RandomCubeTerra;

impl<'a> System<'a> for RandomCubeTerra {
    type SystemData = (Entities<'a>, ReadStorage<'a, ForceTag>);

    fn run(&mut self, (entities, tag): Self::SystemData) {}
}
