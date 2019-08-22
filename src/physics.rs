use amethyst::{
    core::{
        SystemBundle,
    },
    ecs::{
        Read,
        System,
        World,
        DispatcherBuilder,
        Component,
        storage::DenseVecStorage,
    },
    controls::FlyControlTag,
    error::Error,
};

struct CollideBox;

impl Component for CollideBox {
    type Storage = DenseVecStorage<Self>;
}

struct MovementSystem;

impl System for MovementSystem {
    type SystemData = (
    );
    fn run(self, data: Self::SystemData) {
    }
}

struct PhysicsBundle;

impl<'a,'b> SystemBundle<'a,'b> for PhysicsBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
    }
}

