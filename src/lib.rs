use assets::load_assets;
use ecs::World;
use error::IslandResult;
use macroquad::prelude::*;
use prelude::ecs::plugin::Plugin;
use pyo3::Python;

use crate::debug::menu::DebugMenu;

pub mod assets;
pub mod debug;
pub mod ecs;
pub mod error;

pub mod prelude {
    pub use crate::{info, warn, error};
    pub use super::assets::*;
    pub use super::debug::*;
    pub use super::ecs::*;
    pub use super::error::*;
    pub use super::*;
    pub use macroquad::prelude::*;
}

/** A descriptor for an Island-Engine app */
pub struct IEAppDescriptor<'a> {
    // The path to the first scene
    pub first_scene: Option<&'a str>,
    // The path to the asset descriptor
    pub assets_descriptor: Option<&'a str>,
    // The flag for enabling debug mode
    pub debug_mode: bool,
    // The plugins for the app
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl Default for IEAppDescriptor<'_> {
    fn default() -> Self {
        return Self {
            first_scene: None,
            assets_descriptor: None,
            debug_mode: false,
            plugins: vec![],
        };
    }
}

/** Start the game with the scene */
pub async fn start(app_descriptor: IEAppDescriptor<'_>) -> IslandResult<()> {
    // Load the assets
    load_assets(app_descriptor.assets_descriptor).await?;

    // Get the python context
    let _gil = Python::acquire_gil();
    let py = unsafe { Python::assume_gil_acquired() };

    // Create the debug menu
    let mut debug_menu = DebugMenu::new();

    // Create the world
    let mut world = World::new(py, app_descriptor.first_scene, app_descriptor.plugins)?;

    // The debug mode flag
    let debug_mode = app_descriptor.debug_mode;

    // The game loop
    while world.should_run {
        // Update the world
        world.update(debug_mode)?;
        // Clear the background
        clear_background(Color::from_rgba(32, 32, 32, 255));
        // Draw the world
        world.render()?;
        // Draw the debug menu
        if debug_mode {
            debug_menu.draw(&mut world);
        }
        // Move to the next frame
        next_frame().await;
    }
    return Ok(());
}
