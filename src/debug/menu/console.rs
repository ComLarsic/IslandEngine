use std::collections::HashMap;

use egui::Align;
use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::{ecs::World, error, info, warn, debug::log::logger};

type CommandList = HashMap<String, Box<dyn FnMut(String, &mut World)>>;

/** Represents a debug console */
pub struct DebugConsole {
    commands: CommandList,
    current_input: String,
}

impl DebugConsole {
    pub fn new() -> Self {
        return Self {
            commands: Self::register_commands(),
            current_input: "".into(),
        };
    }

    /** Create the command list */
    fn register_commands() -> CommandList {
        let mut list = CommandList::new();

        // Engine manipulation
        list.insert(
            "quit".into(),
            Box::new(|_, world| {
                world.should_run = false;
            }),
        );
        list.insert(
            "toggle_debug_draw".into(),
            Box::new(|_, world| {
                world.debug_draw = !world.debug_draw;
            }),
        );

        // Logging commands
        list.insert(
            "info".into(),
            Box::new(|args, _| {
                info!("{}", args);
            }),
        );
        list.insert(
            "warn".into(),
            Box::new(|args, _| {
                warn!("{}", args);
            }),
        );
        list.insert(
            "error".into(),
            Box::new(|args, _| {
                error!("{}", args);
            }),
        );

        // Scene manipulation commands
        list.insert(
            "spawn".into(),
            Box::new(|args, world| {
                if let Ok(entity) =
                    json::parse(&args).map_err(|e| error!("Failed to spawn entity: {}", e))
                {
                    world.spawn_entity(entity).unwrap();
                }
            }),
        );

        return list;
    }

    /** Draw the debug console */
    pub fn draw(&mut self, world: &mut World, ctx: &egui::CtxRef) {
        egui::Window::new("Console").scroll(true).show(ctx, |ui| {
            // Add log
            ui.text_edit_multiline(&mut logger().log())
                .scroll_to_me(Align::Max);
            // Add input
            ui.text_edit_singleline(&mut self.current_input);
        });

        // Handle the command
        if is_key_pressed(KeyCode::Enter) {
            if let Some(input) = self.current_input.split_once(" ") {
                if let Some(command) = self.commands.get_mut(input.0) {
                    command(input.1.into(), world);
                } else {
                    error!("Invalid command: \"{}\"", input.0);
                }
            } else {
                if let Some(command) = self.commands.get_mut(&self.current_input) {
                    command("".into(), world);
                } else {
                    error!("Invalid command: \"{}\"", self.current_input);
                }
            }
            self.current_input = String::new();
        }
    }
}
