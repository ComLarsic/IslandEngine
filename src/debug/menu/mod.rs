use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::ecs::World;

use self::console::DebugConsole;

pub mod console;

/** Handles the debug menu */
pub struct DebugMenu {
    drawing: bool,
    console: DebugConsole,
}

impl DebugMenu {
    /** Construct a new debug menu */
    pub fn new() -> Self {
        return Self {
            drawing: false,
            console: DebugConsole::new(),
        };
    }

    /** Draw the debug menu */
    pub fn draw(&mut self, world: &mut World) {
        // Draw the log
        if self.drawing {
            egui_macroquad::ui(|ctx| {
                self.console.draw(world, ctx);
            });
            // Draw the egui menu's
            egui_macroquad::draw();
        }

        // Toggle the menu
        if is_key_pressed(KeyCode::F3) {
            self.drawing = !self.drawing;
        }
    }
}
