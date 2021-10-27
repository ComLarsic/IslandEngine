//! A plugin system for island engine
//! For if you need rust's speed or need to render something in a way ie doesn't provide on its own
use crate::prelude::IslandResult;
use super::World;

/** 
To be implemented by each plugin for the island engine.
For if you need rust's speed or need to render something in a way ie doesn't provide on its own.

```no_run
pub struct ExamplePlugin;

impl ie::Plugin for ExamplePlugin {
    fn update(&mut self, world: &mut World) -> IslandResult<()> {
        println!("Hello, world!");
        return Ok(());
    }
}

#[macroquad::main("Plugin example")]
async fn main() -> IslandResult<()> {
    ie::start(IEAppDescriptor::default()).await?;
    return Ok(());
}
```
*/
pub trait Plugin {
    /** To be called on scene initialization */
    fn init(&mut self, _world: &mut World) -> IslandResult<()>;
    /** To be called each update loop */
    fn update(&mut self, _world: &mut World) -> IslandResult<()>;
    /** To be called each render loop */
    fn render(&self, _world: &World) -> IslandResult<()>;
}