use crate::{ecs::World, error::IslandResult};
use macroquad::prelude::{is_key_pressed, KeyCode};

/** Handles scene/script reloading */
pub(crate) fn reload_systems(world: &mut World) -> IslandResult<()> {
    // Reload the scripts
    if is_key_pressed(KeyCode::F4) {
        world.reload_scripts()?;
    }
    // Reload the scene
    if is_key_pressed(KeyCode::F5) {
        world.reload_scene()?;
    }
    return Ok(());
}
