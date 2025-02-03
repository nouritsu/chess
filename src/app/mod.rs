// Modules
mod camera;
mod cursor;
mod render_layer;
mod window;

// Exports
pub use camera::MainCamera;
pub use cursor::{CursorPosition, CursorState};
pub use render_layer::RenderLayer;
pub use window::{WINDOW_HEIGHT, WINDOW_WIDTH};

// Imports
use crate::{board, handler, turn};
use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct AppPlugin;

impl PluginGroup for AppPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(window::plugin)
            .add(camera::plugin)
            .add(cursor::plugin)
            .add(render_layer::plugin)
            .add_group(handler::HandlerPlugin)
            .add_group(board::BoardPlugin)
            .add_group(turn::TurnPlugin)
    }
}
