use amethyst::{
    assets::{PrefabLoader, PrefabLoaderSystemDesc, RonFormat},
    controls::{CursorHideSystem, HideCursor, MouseFocusUpdateSystemDesc},
    core::transform::{Transform, TransformBundle},
    input::{is_key_down, is_mouse_button_down, InputBundle, StringBindings, VirtualKeyCode},
    prelude::*,
    renderer::{types::DefaultBackend, Camera, RenderFlat3D, RenderToWindow, RenderingBundle},
    utils::application_root_dir,
};
use winit::MouseButton;

use crate::physics::{PlayerTag, RigidBody};
use crate::prefab::ScenePrefabData;
use crate::terra::{CubeTerraSystemDesc, FlatHeightGenerator};

mod physics;
mod prefab;
mod terra;

struct InWorld;

impl SimpleState for InWorld {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let camera = Camera::standard_3d(1024.0, 768.0);
        world
            .create_entity()
            .named("player")
            .with(RigidBody { size: 2.0 })
            .with(Transform::default())
            .with(PlayerTag)
            .with(camera)
            .build();
        let handle = world.exec(|loader: PrefabLoader<'_, ScenePrefabData>| {
            loader.load("prefab/scene.ron", RonFormat, ())
        });
        world.create_entity().with(handle).build();
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut cursor = data.world.write_resource::<HideCursor>();
        cursor.hide = false;
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut cursor = data.world.write_resource::<HideCursor>();
        cursor.hide = true;
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PauseState));
            }
        }
        Trans::None
    }
}

struct PauseState;

impl SimpleState for PauseState {
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            if is_mouse_button_down(&event, MouseButton::Left) {
                return Trans::Pop;
            }
        }
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let asset_dir = app_root.join("assets");
    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(config_dir.join("input.ron"))?,
        )?
        .with_system_desc(
            PrefabLoaderSystemDesc::<ScenePrefabData>::default(),
            "",
            &[],
        )
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(config_dir.join("display.ron"))
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat3D::default()),
        )?
        .with_system_desc(MouseFocusUpdateSystemDesc, "", &[])
        .with(CursorHideSystem::default(), "", &[])
        .with(physics::PhysicsWorldSystem, "", &[])
        .with(physics::NPhysicsSystem, "physics", &[])
        .with(physics::PlayerInputSystem, "", &[])
        .with_system_desc(physics::PlayerRotateSystemDesc, "", &[])
        .with_system_desc(
            CubeTerraSystemDesc::new(FlatHeightGenerator::new(1.)),
            "",
            &[],
        )
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::new(asset_dir, InWorld, game_data)?;
    game.run();
    Ok(())
}
