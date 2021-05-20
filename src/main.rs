// Pong 
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

mod pong;
mod systems;

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            // RenderToWindow used to draw everything on window
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RednerFlat2D plugin is used to render entities with a 'SpriteRender'
            .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(
            systems::BounceSystem,
            "collison_system",
            &["paddle_system", "ball_system"],
        );
            

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();

    Ok(())
}