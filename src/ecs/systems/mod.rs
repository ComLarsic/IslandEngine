use json::JsonValue;
use macroquad::prelude::{Rect, Vec2};

use crate::prelude::{IslandError, IslandResult};

pub mod debug;
pub mod render;

// Wrapper functions for handeling json data
pub trait JsonConvert {
    /** Get the value as a glam::Vec2 */
    fn as_vec2(&self) -> IslandResult<Vec2>;
    /** Get the value as a macroquad::math::Rect */
    fn as_rect(&self) -> IslandResult<Rect>;
}

impl JsonConvert for JsonValue {
    fn as_vec2(&self) -> IslandResult<Vec2> {
        let x = self["x"]
            .as_f32()
            .ok_or(IslandError::new(format!("Failed to convert: {} to vec2", self.to_string())))?;
        let y = self["y"]
            .as_f32()
            .ok_or(IslandError::new(format!("Failed to convert: {} to vec2", self.to_string())))?;

        return Ok(Vec2::new(x, y));
    }

    fn as_rect(&self) -> IslandResult<Rect> {
        let x = self["x"]
            .as_f32()
            .ok_or(IslandError::new(format!("Failed to convert: {} to rect", self.to_string())))?;
        let y = self["y"]
            .as_f32()
            .ok_or(IslandError::new(format!("Failed to convert: {} to rect", self.to_string())))?;
        let w = self["w"]
            .as_f32()
            .ok_or(IslandError::new(format!("Failed to convert: {} to rect", self.to_string())))?;
        let h = self["h"]
            .as_f32()
            .ok_or(IslandError::new(format!("Failed to convert: {} to rect", self.to_string())))?;

        return Ok(Rect::new(x, y, w, h));
    }
}
