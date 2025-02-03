// Modules
mod common;
mod execute_move;
mod highlight_move;
mod selector;

// Exports
pub use selector::Selector;

// Imports
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct TurnPlugin;

impl PluginGroup for TurnPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(selector::plugin)
            .add(highlight_move::plugin)
            .add(execute_move::plugin)
    }
}
