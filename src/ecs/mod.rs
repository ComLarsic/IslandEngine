#[allow(unused_must_use)]
use crate::{
    error,
    error::{IslandError, IslandResult},
    info,
};
use std::path::PathBuf;
use json::JsonValue;
use pyo3::prelude::*;

use self::systems::{
    debug::reload_systems,
    render::{handle_camera, render_sprites, render_transforms},
};
pub use plugin::Plugin;

pub mod ctx;
pub mod systems;
pub mod plugin;

/** Handles teh current world state */
#[derive(Debug, Clone, Copy)]
pub enum WorldState {
    Editor,
    Menu,
    Gameplay,
}

/** Represents an ecs world */
pub struct World<'a> {
    // The scene as a json value
    pub scene: JsonValue,
    // The state of the scene when it was loaded
    initial_scene: JsonValue,
    // The scripts
    pub scripts: Option<Vec<&'a PyModule>>,
    pub py_json: &'a PyModule,

    // The python context
    py: Python<'a>,

    // The plugins
    plugins: Option<Vec<Box<dyn Plugin>>>,

    // The flag for if the game should be running
    pub should_run: bool,
    // The current gamestate
    pub state: WorldState,
    // The flag for if debug-rendering should be enabled
    pub debug_draw: bool,
}

impl<'a> World<'a> {
    /** Construct a new world */
    pub fn new(py: Python<'a>, first_scene: Option<&str>, plugins: Vec<Box<dyn Plugin>>) -> IslandResult<Self> {
        // Create the world
        let mut result = Self {
            scene: JsonValue::Null,
            initial_scene: JsonValue::Null,
            scripts: Some(vec![]),
            py: py,
            plugins: Some(plugins),
            py_json: PyModule::import(py, "json").unwrap(),
            should_run: true,
            state: WorldState::Gameplay,
            debug_draw: false,
        };
        // Load the first scene if it is provided
        if let Some(first_scene) = first_scene {
            result.load_scene(first_scene)?;
        } else {
            // Create an empty scene
            result.scene = json::object! {
                camera: {
                    position: {
                        x: 0,
                        y: 0,
                    },
                    zoom: {
                        x: 1,
                        y: 1,
                    }
                },
                entities: []
            }
        }

        return Ok(result);
    }

    /** Load a scene into the world */
    pub fn load_scene<T>(&mut self, path: T) -> IslandResult<()>
    where
        T: Into<PathBuf> + Clone,
    {
        // Open the file
        let file = std::fs::read_to_string(path.clone().into()).map_err(|e| {
            return IslandError::new(format!(
                "Failed to load scene \"{:?}\": {}",
                path.clone().into(),
                e
            ));
        })?;
        // Parse the scene
        self.scene = json::parse(&file).map_err(|e| {
            return IslandError::new(format!(
                "Failed to parse scene \"{:?}\": {}",
                path.clone().into(),
                e
            ));
        })?;
        // Set the initial scene state
        self.initial_scene = self.scene.clone();
        // Load the scripts
        self.scripts = Some(self.load_scripts()?);

                // Call the plugins
        let mut plugins = self.plugins.take().unwrap();
        for plugin in plugins.iter_mut() {
            plugin.init(self)?;
        }
        self.plugins = Some(plugins);

        info!("Loaded scene file: {:?}", path.clone().into());

        return Ok(());
    }

    /** Load a scene from a string */
    pub fn load_scene_str<T>(&mut self, scene: T) -> IslandResult<()>
    where
        T: Into<String>,
    {
        // Parse the scene
        self.scene = json::parse(&scene.into()).map_err(|e| {
            return IslandError::new(format!("Failed to parse scene: {}", e));
        })?;
        // Load the scripts
        self.scripts = Some(self.load_scripts()?);

                // Call the plugins
        let mut plugins = self.plugins.take().unwrap();
        for plugin in plugins.iter_mut() {
            plugin.init(self)?;
        }
        self.plugins = Some(plugins);
        return Ok(());
    }

    /** Load a scene from a json object */
    pub fn load_scene_json<T>(&mut self, scene: T) -> IslandResult<()>
    where
        T: Into<JsonValue>,
    {
        // Parse the scene
        self.scene = scene.into();
        // Load the scripts
        self.scripts = Some(self.load_scripts()?);

                // Call the plugins
        let mut plugins = self.plugins.take().unwrap();
        for plugin in plugins.iter_mut() {
            plugin.init(self)?;
        }
        self.plugins = Some(plugins);
        return Ok(());
    }

    /** Reload the scene */
    pub fn reload_scene(&mut self) -> IslandResult<()> {
        self.scene = self.initial_scene.clone();
        // Load the scripts
        self.scripts = Some(self.load_scripts()?);

                // Call the plugins
        let mut plugins = self.plugins.take().unwrap();
        for plugin in plugins.iter_mut() {
            plugin.init(self)?;
        }
        self.plugins = Some(plugins);
        info!("Reloaded scene");
        return Ok(());
    }

    /** Reload the scripts */
    pub fn reload_scripts(&mut self) -> IslandResult<()> {
        // Load the scripts
        self.scripts = Some(self.load_scripts()?);
        info!("Reloaded scripts");
        return Ok(());
    }

    /** Update the world */
    pub fn update(&mut self, debug_mode: bool) -> IslandResult<()> {
        let scripts = self.scripts.take().unwrap();
        // Update the scripts
        for script in scripts.iter() {
            self.run_script_stage(script, "update")?;
        }
        self.scripts = Some(scripts);

        // Call the debug systems
        if debug_mode {
            reload_systems(self)?;
        }

        // Call the update systems

        // Call the plugins
        let mut plugins = self.plugins.take().unwrap();
        for plugin in plugins.iter_mut() {
            plugin.update(self)?;
        }
        self.plugins = Some(plugins);

        return Ok(());
    }

    /** Render the world */
    pub fn render(&mut self) -> IslandResult<()> {
        // Call rendering systems
        handle_camera(&self.scene)?;
        render_sprites(&self.scene)?;
        // Call the plugins
        let mut plugins = self.plugins.take().unwrap();
        for plugin in plugins.iter_mut() {
            plugin.render(self)?;
        }
        self.plugins = Some(plugins);
        // Call the debug render systems
        if self.debug_draw {
            render_transforms(&self.scene)?;
        }
        return Ok(());
    }

    /** Run a python script */
    fn run_script_stage(&mut self, script: &PyModule, stage: &str) -> IslandResult<()> {
        // Get the update functions
        let func = script.getattr(stage).map_err(|_| {
            return IslandError::new(format!(
                "Failed to find {} functions in \"{}\"",
                stage,
                script.name().unwrap()
            ));
        })?;

        // Create the context
        let ctx = PyCell::new(self.py, ctx::HighgroundCtx::new()).map_err(|e| {
            return IslandError::new(format!("Failed to create highground ctx: {}", e));
        })?;

        // Convert the scene to a python json
        let scene_py = json_to_py(&self.scene, self.py_json)?;
        // Call the update function and get the scene
        let returned_scene = func.call1((ctx.borrow_mut(), scene_py)).map_err(|e| {
            return IslandError::new(format!("Script error: {}, {}", script.name().unwrap(), e));
        })?;

        // Set the new scene
        self.scene = py_to_json(returned_scene, self.py_json)?;

        // Modify the world by passing the context
        self.handle_context(ctx.borrow())?;

        return Ok(());
    }

    /** Handle the highground context after its been returned */
    pub fn handle_context(&mut self, ctx: PyRef<ctx::HighgroundCtx>) -> IslandResult<()> {
        // Spawn the entities
        for entity in &ctx.spawned_entities {
            // Strange if-block to prevent the compiler from complaining because i am not using the error
            if let Ok(_) = self.spawn_entity(entity.clone()).map_err(|e| {
                error!("Failed to spawn entity: {}", e);
            }) {}
        }

        return Ok(());
    }

    /** Spawn an entity */
    pub fn spawn_entity(&mut self, entity: JsonValue) -> IslandResult<()> {
        // Add the entity
        self.scene["entities"].push(entity).map_err(|e| {
            return IslandError::new(format!("Failed to push an entity to the world: {}", e));
        })?;
        return Ok(());
    }

    /** Load a scenes scripts */
    fn load_scripts(&mut self) -> IslandResult<Vec<&'a PyModule>> {
        // The scripts
        let mut scripts = vec![];

        let scene = self.scene.clone();

        // Load the scripts
        if scene.has_key("scripts") {
            for script in scene["scripts"].members() {
                // Get the script path
                let path = script
                    .as_str()
                    .ok_or(IslandError::new("Cannot parse script path as string"))?;
                // Get the name
                let name = path.split("/").collect::<Vec<&str>>();
                let name = name.last().unwrap().replace(".py", "");
                // Read the script
                let code = std::fs::read_to_string(path).map_err(|e| {
                    return IslandError::new(format!(
                        "Failed to read script \"{:?}\": {}",
                        path.clone(),
                        e
                    ));
                })?;
                // Create the script module
                let module = PyModule::from_code(self.py, &code, &format!("{}.py", name), &name)
                    .map_err(|e| {
                        return IslandError::new(format!(
                            "Failed to read script \"{:?}\": {}",
                            path.clone(),
                            e
                        ));
                    })?;

                // Call the init function
                self.run_script_stage(module, "init")?;
                // Push the script
                scripts.push(module);
            }
        }

        return Ok(scripts);
    }
}

/** Convert a json object to a python-compatible json object  */
fn json_to_py<'a>(json: &JsonValue, py_json: &'a PyModule) -> IslandResult<&'a PyAny> {
    let convert_fn = py_json.getattr("loads").map_err(|e| {
        return IslandError::new(format!(
            "Failed to fetch `loads` function in json module: {}",
            e
        ));
    })?;
    // Convert the json to a string
    let json_str = json::stringify(json.clone());
    let result = convert_fn.call1((json_str,)).map_err(|e| {
        return IslandError::new(format!(
            "Failed to convert the json object to a python-json object: {}",
            e
        ));
    })?;

    return Ok(result);
}

/** Convert a python-json object to a rust-json object */
fn py_to_json<'a>(obj: &'a PyAny, py_json: &'a PyModule) -> IslandResult<JsonValue> {
    let convert_fn = py_json.getattr("dumps").map_err(|e| {
        return IslandError::new(format!(
            "Failed to fetch `dumps` function in json module: {}",
            e
        ));
    })?;
    // Call the dump functio
    let result = convert_fn.call1((obj,)).map_err(|e| {
        return IslandError::new(format!(
            "Failed to convert python-json object into json-object: {}",
            e
        ));
    })?;
    // Cast to string
    let result_str: String = result.extract().map_err(|e| {
        return IslandError::new(format!(
            "Failed to convert python-json result to string: {}",
            e
        ));
    })?;
    // Parse the resulted strung
    let result_parsed = json::parse(&result_str).map_err(|e| {
        return IslandError::new(format!("Failed to parse python-json string: {}", e));
    })?;

    return Ok(result_parsed);
}
