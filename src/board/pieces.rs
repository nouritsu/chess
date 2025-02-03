use crate::{
    app::RenderLayer,
    board::{BoardPosition, ChessBoard, BOARD_SIZE},
    handler::{Handler, PieceSpriteHandler},
};
use bevy::prelude::*;
use pleco::{Piece, SQ};

pub const PIECE_SIZE: f32 = 48.;

fn spawn_pieces(
    mut cmd: Commands,
    sprite_handler: Res<PieceSpriteHandler>,
    board: Res<ChessBoard>,
) {
    let mut spawn_piece = |p: u8, piece_type: Piece| {
        let sprite = Sprite {
            image: sprite_handler.get(piece_type),
            custom_size: Some(Vec2::splat(PIECE_SIZE)),
            ..Default::default()
        };

        let layer = RenderLayer::Pieces;

        let board_pos = BoardPosition(SQ::from(p));

        cmd.spawn((board_pos, sprite, layer));
    };

    for i in 0..(BOARD_SIZE * BOARD_SIZE) {
        let pieces = board.get_piece_locations();
        match pieces.piece_at(SQ::from(i)) {
            Piece::None => continue,
            piece => spawn_piece(i, piece),
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(PostStartup, spawn_pieces);
}
