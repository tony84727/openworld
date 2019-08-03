use amethyst::{
    assets::{PrefabLoader, PrefabLoaderSystemDesc, RonFormat},
    core::transform::{Transform, TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        rendy::mesh::{Normal, Position, TexCoord},
        types::DefaultBackend,
        Camera, RenderFlat3D, RenderSkybox, RenderToWindow, RenderingBundle,
    },
    utils::{application_root_dir, scene::BasicScenePrefab},
};
mod component;
mod input;
struct InWorld;

impl SimpleState for InWorld {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let camera = Camera::standard_3d(1024.0, 768.0);
        world
            .create_entity()
            .with(Transform::default())
            .with(component::Player::new())
            .with(camera)
            .build();
        let handle = world.exec(|loader: PrefabLoader<'_, ScenePrefabData>| {
            loader.load("prefab/scene.ron", RonFormat, ())
        });
        world.create_entity().with(handle).build();
    }
}

type ScenePrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let asset_dir = app_root.join("assets");
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(config_dir.join("input.ron"))?,
        )?
        .with(input::MovementSystem, "", &[])
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
                .with_plugin(RenderSkybox::default())
                .with_plugin(RenderFlat3D::default()),
        )?;
    let mut game = Application::new(asset_dir, InWorld, game_data)?;
    game.run();
    Ok(())
}
