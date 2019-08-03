use crate::component::Player;
use amethyst::{
    core::{math::*, Transform},
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
    window::Window,
    winit::dpi::LogicalPosition,
};
use std::f32::consts::PI;

pub struct MovementSystem;

fn mouse_axis(window: &Window, input_handler: &InputHandler<StringBindings>) -> Option<(f32, f32)> {
    if let Some((x, y)) = input_handler.mouse_position() {
        if let Some(size) = window.get_inner_size() {
            let x = (x - size.width as f32 / 2.0) / size.width as f32 * PI;
            let y = (y - size.height as f32 / 2.0) / size.width as f32 * PI;
            return Some((-x, -y));
        }
    }
    None
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        WriteExpect<'s, Window>,
    );

    fn run(&mut self, (mut transform, player, input, window): Self::SystemData) {
        for (_player, transform) in (&player, &mut transform).join() {
            let horizontal = input.axis_value("horizontal").unwrap_or(0.0);
            let vertical = input.axis_value("vertical").unwrap_or(0.0);
            transform.move_right(horizontal);
            transform.move_forward(vertical);
            if let Some((x, y)) = mouse_axis(&window, &input) {
                transform.set_rotation(UnitQuaternion::from_euler_angles(y, x, 0.0));
            }
        }
    }
}
