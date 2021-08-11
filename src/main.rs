use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};

mod go;
mod systems;
mod utility;

use crate::go::Go;

fn main() -> amethyst::Result<()> {
    //App prelude
    let app_root = application_root_dir()?;
    amethyst::start_logger(Default::default());

    // Input bindings
    let binding_path = app_root.join("config").join("bindings.ron");    
    let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(binding_path)?;

    //Display config
    let display_config_path = app_root.join("config").join("display.ron");
    
    //Game Init
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default()),
      )?
      .with_bundle(TransformBundle::new())?
      .with_bundle(input_bundle)?
      .with(systems::MessageSystem::default(), "message_system", &["input_system"])
      .with(systems::StoneSystem::new(), "stone_system", &["message_system", "input_system"]);

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Go::default(), game_data)?;
    game.run();

    Ok(())
}