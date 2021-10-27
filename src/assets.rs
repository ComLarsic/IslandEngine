use std::path::PathBuf;

use error::IslandError;
use json::JsonValue;
use macroquad::prelude::*;

use crate::{error, error::IslandResult};

/** The default sprite */
const DEFAILT_SPRITE: &[u8] = include_bytes!("default_sprite.png");

/** The singleton for storing sprites */
static mut SPRITES: Option<hashbrown::HashMap<String, Texture2D>> = None;

/** Get the sprites singleton */
pub fn sprites() -> &'static mut hashbrown::HashMap<String, Texture2D> {
    unsafe {
        if SPRITES.is_none() {
            SPRITES = Some(hashbrown::HashMap::new());
        }
        return SPRITES.as_mut().unwrap();
    };
}

/** Load the games assets */
pub(crate) async fn load_assets<T>(asset_descriptor: Option<T>) -> IslandResult<()>
where
    T: Into<PathBuf> + Clone,
{
    if let Some(asset_descriptor) = asset_descriptor {
        // Read the asset descriptor
        let asset_descriptor = std::fs::read_to_string(asset_descriptor.into()).map_err(|e| {
            return IslandError::new(format!("Failed to read asset descriptor: {}", e));
        })?;
        let asset_descriptor = json::parse(&asset_descriptor).map_err(|e| {
            return IslandError::new(format!("Failed to parse asset descriptor: {}", e));
        })?;

        // Load the sprites
        load_sprites(Some(&asset_descriptor)).await?;
        return Ok(());
    }

    // Load only the default sprites
    load_sprites(None).await?;

    return Ok(());
}

/** Load the sprites */
async fn load_sprites(asset_descriptor: Option<&JsonValue>) -> IslandResult<()> {
    let mut sprites = hashbrown::HashMap::new();
    // Add default sprite
    let default = Texture2D::from_file_with_format(DEFAILT_SPRITE, None);
    default.set_filter(FilterMode::Nearest);
    sprites.insert("default_sprite".into(), default);
    if let Some(asset_descriptor) = asset_descriptor {
        // Get the sprites values
        let sprites_descripor = asset_descriptor["sprites"].members();

        // Load the sprites
        for sprite_descriptor in sprites_descripor {
            let name = &sprite_descriptor["name"].as_str().ok_or(|| {
                error!("failed to load sprite, no name provided");
            });
            let path = &sprite_descriptor["path"].as_str().ok_or(|| {
                error!("failed to load sprite, no path provided");
            });
            let filter = &sprite_descriptor["filter"].as_str().ok_or(|| {
                error!("failed to load sprite, no filter provided");
            });

            if let (Ok(name), Ok(path), Ok(filter)) = (name, path, filter) {
                // Decide the filter
                let filter = match filter {
                    &"linear" => FilterMode::Linear,
                    &"nearest" => FilterMode::Nearest,
                    _ => {
                        error!("Invalid filter, defaulting to nearest");
                        FilterMode::Nearest
                    }
                };

                // Add the sprite
                sprites.insert((*name).into(), load_sprite(path, filter).await?);
            }
        }
    }
    // Set the singleton
    unsafe { SPRITES = Some(sprites) };
    return Ok(());
}

/** A wrapper function for laoding a sprote */
async fn load_sprite(path: &str, filter: FilterMode) -> IslandResult<Texture2D> {
    if let Ok(texture) = load_texture(path).await {
        texture.set_filter(filter);
        return Ok(texture);
    }
    error!("Failed to load sprite \"{}\"", path);
    return Ok(Texture2D::from_file_with_format(DEFAILT_SPRITE, None));
}

/** Get a sprite */
pub fn get_sprite(name: &str) -> &'static Texture2D {
    if let Some(texture) = sprites().get(name) {
        return texture;
    }
    error!("No such sprite: {}", name);
    return sprites().get("default_sprite").unwrap();
}
