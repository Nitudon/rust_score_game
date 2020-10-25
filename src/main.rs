use amethyst::{
    {GameDataBuilder, Application},
    utils::application_root_dir,
    core::TransformBundle,
    renderer::{RenderingBundle, RenderToWindow, RenderFlat2D, types::DefaultBackend}, 
};
use crate::bundle::GameBundle;
use crate::state::state::GameState;

mod bundle;
mod asset;
mod input;
mod components;
mod state;
mod systems;
mod util;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(GameBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?.with_clear([0.,0.,0.,1.])
            )
            .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(input::bundle::create_input_bundle())?;

    Application::new(assets_dir, GameState, game_data)?.run();
    Ok(())
}
