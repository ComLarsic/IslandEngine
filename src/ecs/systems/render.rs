use crate::{assets::get_sprite, error::IslandResult};
use json::JsonValue;
use macroquad::prelude::*;

use super::JsonConvert;

/** Render the transforms */
pub(crate) fn render_transforms(scene: &JsonValue) -> IslandResult<()> {
    for entity in scene["entities"].members() {
        if entity.has_key("position") && entity.has_key("scale") {
            // Get the components
            let position = &entity["position"];
            let scale = &entity["scale"];

            draw_rectangle(
                position["x"]
                    .as_f32()
                    .expect("position's x value should be a f32"),
                position["y"]
                    .as_f32()
                    .expect("position's y value should be a f32"),
                16.0 * scale["x"]
                    .as_f32()
                    .expect("scale's x value should be a f32"),
                16.0 * scale["y"]
                    .as_f32()
                    .expect("scale's y value should be a f32"),
                GRAY,
            );
        }
    }

    return Ok(());
}

/** Render the sprites */
pub(crate) fn render_sprites(scene: &JsonValue) -> IslandResult<()> {
    for entity in scene["entities"].members() {
        if entity.has_key("position") && entity.has_key("scale") && entity.has_key("sprite") {
            // Get the components
            let position = entity["position"].as_vec2()?;
            let scale = entity["scale"].as_vec2()?;
            let sprite = &entity["sprite"];

            let name = sprite["texture"].as_str().unwrap();
            let dest = sprite["dest_size"].as_vec2()?;
            let source = sprite["source_rec"].as_rect();
            let flip_x = sprite["flip_x"].as_bool().unwrap();
            let flip_y = sprite["flip_y"].as_bool().unwrap();
            let texture = get_sprite(name);

            draw_texture_ex(
                *texture,
                position.x,
                position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(dest * scale),
                    source: if let Ok(source) = source {
                        Some(source)
                    } else {
                        None
                    },
                    rotation: 0.0,
                    flip_x,
                    flip_y,
                    pivot: None,
                },
            );
        }
    }

    return Ok(());
}

/** Handle the camera on the scene */
pub(crate) fn handle_camera(scene: &JsonValue) -> IslandResult<()> {
    let camera = &scene["camera"];
    let position = camera["position"].as_vec2()?;
    let zoom = &camera["zoom"].as_vec2()?;
    let zoom = (
        zoom.x / screen_width() * 2.0,
        zoom.y / -screen_height() * 2.0,
    );

    set_camera(&Camera2D {
        rotation: 0.0,
        zoom: zoom.into(),
        target: position,
        ..Default::default()
    });

    return Ok(());
}
