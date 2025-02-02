// Modules
mod board_position;
mod chessboard;
mod pieces;

// Exports
pub use board_position::BoardPosition;
pub use chessboard::{ChessBoard, BOARD_SIZE, CELL_SIZE};
pub use pieces::PIECE_SIZE;

// Imports
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct BoardPlugin;

impl PluginGroup for BoardPlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(board_position::plugin)
            .add(chessboard::plugin)
            .add(pieces::plugin)
    }
}
