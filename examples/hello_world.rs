use ie::prelude::*;

/** Returns the window config */
pub fn window_conf() -> Conf {
    return Conf {
        window_title: "Hello IE!".into(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    };
}

/** An example plugin */
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn init(&mut self, _world: &mut World) -> IslandResult<()> {
        info!("Hello, plugin!");
        return Ok(());
    }

    fn update(&mut self, _world: &mut World) -> IslandResult<()> {
        return Ok(());
    }

    fn render(&self, _world: &World) -> IslandResult<()> {
        // Custom rendering using macroquad
        draw_text("Hello, world!", -(1280.0 / 2.0), -(720.0 / 2.0) + 32.0, 32.0, GREEN);
        return Ok(());
    }
}

/** The macroquad entrypoint */
#[macroquad::main(window_conf)]
async fn main() -> IslandResult<()> {
    // Set the world with the first loaded scene
    ie::start(IEAppDescriptor {
        first_scene: Some("examples/example_scene.json"),
        assets_descriptor: Some("examples/assets.json"),
        debug_mode: true,
        plugins: vec![Box::new(HelloPlugin)],
        ..Default::default()
    })
    .await?;
    return Ok(());
}
