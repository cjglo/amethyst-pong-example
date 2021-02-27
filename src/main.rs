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


pub struct Pong;

impl SimpleState for Pong { }

fn main() -> Result<()> {

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let game_data = GameDataCuilder::default();

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();

    // left off an "Creating an application"

    // theory left off at "EntitiesRes"


    Ok(())
}