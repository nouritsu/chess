// Modules
mod color_handler;
mod common;
mod handler;
mod piece_sprite_handler;

// Exports
use bevy::{app::PluginGroupBuilder, prelude::*};
pub use color_handler::{ColorHandler, GameColor};
pub use common::AsAssetPath;
pub use handler::Handler;
pub use piece_sprite_handler::PieceSpriteHandler;

pub struct HandlerPlugin;

impl PluginGroup for HandlerPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(color_handler::plugin)
            .add(piece_sprite_handler::plugin)
    }
}
