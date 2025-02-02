// Modules
mod common;
mod execute_move;
mod highlight_move;
mod selector;

// Exports
pub use highlight_move::HighlightState;
pub use selector::{Selector, SelectorFromSelected};

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
