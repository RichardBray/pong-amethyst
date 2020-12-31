use amethyst::{
    prelude::*, // Includes basic types - Application, World, and State
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
};

mod pong;
mod systems;

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    // Amethyst logger so we can see warnings, errors, and debug messages in the terminal
    amethyst::start_logger(Default::default());
    
    // Linking up the display settings from config/display.ron to game
    let app_root = application_root_dir()?; // ?
    let display_config_path = app_root.join("config/display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");
    let assets_dir = app_root.join("assets");

    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    // Creating application
    let game_data = GameDataBuilder::default()
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]), // Background colourin RGBA
            )
            // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
            .with_plugin(RenderFlat2D::default()),
    )?
    .with_bundle(TransformBundle::new())?
    .with_bundle(input_bundle)?
    .with(systems::PaddleSystem, "paddle_system", &["input_system"]);

    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    Ok(())
}
