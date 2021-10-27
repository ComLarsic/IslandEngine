use json::JsonValue;
use macroquad::prelude::*;
use pyo3::prelude::*;

use crate::{error, info, warn};

use super::WorldState;

/** The context that gets passed into the python scripts */
#[pyclass]
pub(crate) struct HighgroundCtx {
    pub(crate) world_state: WorldState,
    pub(crate) spawned_entities: Vec<JsonValue>,
}

#[pymethods]
impl HighgroundCtx {
    // Timing info

    /** Get the delta time */
    pub fn delta_time(&self) -> PyResult<f32> {
        return Ok(get_frame_time());
    }

    /** Get the fps */
    pub fn fps(&self) -> PyResult<i32> {
        return Ok(get_fps());
    }

    /** Get the frame time */
    pub fn frame_time(&self) -> PyResult<f32> {
        return Ok(get_frame_time());
    }

    // Logging functions

    /** Push an info log */
    pub fn log_info(&self, log: String) -> PyResult<()> {
        info!("{}", log);
        return Ok(());
    }

    /** Push a warning log */
    pub fn log_warn(&self, log: String) -> PyResult<()> {
        warn!("{}", log);
        return Ok(());
    }

    /** Push an error log */
    pub fn log_error(&self, log: String) -> PyResult<()> {
        error!("{}", log);
        return Ok(());
    }

    // Input handeling

    /** Check if the key is pressed on the current frame */
    pub fn is_key_pressed(&self, key: String) -> PyResult<bool> {
        return Ok(is_key_pressed(string_to_keycode(&key)));
    }

    /** Check if the key is released on the current frame */
    pub fn is_key_released(&self, key: String) -> PyResult<bool> {
        return Ok(is_key_released(string_to_keycode(&key)));
    }

    /** Check if the key id held down */
    pub fn is_key_down(&self, key: String) -> PyResult<bool> {
        return Ok(is_key_down(string_to_keycode(&key)));
    }

    // Scene manipulation
    /** Spawn an entity in the scene */
    pub fn spawn_entity(&mut self, entity_input: String) -> PyResult<()> {
        if let Ok(entity) = json::parse(&entity_input).map_err(|e| {
            error!("Failed to spawn entity: {}", e);
        }) {
            info!("spawing: {}", entity_input);
            self.spawned_entities.push(entity);
        }
        return Ok(());
    }

    /** Get the world state */
    pub fn world_state(&self) -> PyResult<String> {
        return Ok(match self.world_state {
            WorldState::Editor => "Editor",
            WorldState::Menu => "Menu",
            WorldState::Gameplay => "Gameplay",
        }.into());
    }
}

/** Convert a u32 to a keycode */
fn string_to_keycode(key: &str) -> KeyCode {
    use KeyCode::*;
    return match key {
        "Space" => Space,
        "Apostrophe" => Apostrophe,
        "Comma" => Comma,
        "Minus" => Minus,
        "Period" => Period,
        "Slash" => Slash,
        "Key0" => Key0,
        "Key1" => Key1,
        "Key2" => Key2,
        "Key3" => Key3,
        "Key4" => Key4,
        "Key5" => Key5,
        "Key6" => Key6,
        "Key7" => Key7,
        "Key8" => Key8,
        "Key9" => Key9,
        "Semicolon" => Semicolon,
        "Equal" => Equal,
        "A" => A,
        "B" => B,
        "C" => C,
        "D" => D,
        "E" => E,
        "F" => F,
        "G" => G,
        "H" => H,
        "I" => I,
        "J" => J,
        "K" => K,
        "L" => L,
        "M" => M,
        "N" => N,
        "O" => O,
        "P" => P,
        "Q" => Q,
        "R" => R,
        "S" => S,
        "T" => T,
        "U" => U,
        "V" => V,
        "W" => W,
        "X" => X,
        "Y" => Y,
        "Z" => Z,
        "LeftBracket" => LeftBracket,
        "Backslash" => Backslash,
        "RightBracket" => RightBracket,
        "GraveAccent" => GraveAccent,
        "World1" => World1,
        "World2" => World2,
        "Escape" => Escape,
        "Enter" => Enter,
        "Tab" => Tab,
        "Backspace" => Backspace,
        "Insert" => Insert,
        "Delete" => Delete,
        "Right" => Right,
        "Left" => Left,
        "Down" => Down,
        "Up" => Up,
        "PageUp" => PageUp,
        "PageDown" => PageDown,
        "Home" => Home,
        "End" => End,
        "CapsLock" => CapsLock,
        "ScrollLock" => ScrollLock,
        "NumLock" => NumLock,
        "PrintScreen" => PrintScreen,
        "Pause" => Pause,
        "F1" => F1,
        "F2" => F2,
        "F3" => F3,
        "F4" => F4,
        "F5" => F5,
        "F6" => F6,
        "F7" => F7,
        "F8" => F8,
        "F9" => F9,
        "F10" => F10,
        "F11" => F11,
        "F12" => F12,
        "F13" => F13,
        "F14" => F14,
        "F15" => F15,
        "F16" => F16,
        "F17" => F17,
        "F18" => F18,
        "F19" => F19,
        "F20" => F20,
        "F21" => F21,
        "F22" => F22,
        "F23" => F23,
        "F24" => F24,
        "F25" => F25,
        "Kp0" => Kp0,
        "Kp1" => Kp1,
        "Kp2" => Kp2,
        "Kp3" => Kp3,
        "Kp4" => Kp4,
        "Kp5" => Kp5,
        "Kp6" => Kp6,
        "Kp7" => Kp7,
        "Kp8" => Kp8,
        "Kp9" => Kp9,
        "KpDecimal" => KpDecimal,
        "KpDivide" => KpDivide,
        "KpMultiply" => KpMultiply,
        "KpSubtract" => KpSubtract,
        "KpAdd" => KpAdd,
        "KpEnter" => KpEnter,
        "KpEqual" => KpEqual,
        "LeftShift" => LeftShift,
        "LeftControl" => LeftControl,
        "LeftAlt" => LeftAlt,
        "LeftSuper" => LeftSuper,
        "RightShift" => RightShift,
        "RightControl" => RightControl,
        "RightAlt" => RightAlt,
        "RightSuper" => RightSuper,
        "Menu" => Menu,
        _ => Unknown,
    };
}
