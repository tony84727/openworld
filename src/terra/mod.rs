use amethyst::ecs::{Entities, ReadStorage, System};

struct RandomCubeTerra;

impl<'a> System<'a> for RandomCubeTerra {
    type SystemData = Entities<'a>;

    fn run(&mut self, entities: Self::SystemData) {}
}
