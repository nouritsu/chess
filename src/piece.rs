use crate::{
    board::{ChessBoard, BOARD_SIZE},
    board_position::BoardPosition,
    handler::{Handler, PieceSpriteHandler},
    render_layer::RenderLayer,
};
use bevy::prelude::*;
use pleco::{Piece, SQ};
use strum::VariantArray;

pub const PIECE_SIZE: f32 = 48.;

fn init_handler(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let mut sprite_handler = PieceSpriteHandler::new();

    for piece in Piece::VARIANTS.iter().filter(|&&p| p != Piece::None) {
        let path = match piece {
            Piece::WhitePawn => "white/pawn.png",
            Piece::WhiteKnight => "white/knight.png",
            Piece::WhiteBishop => "white/bishop.png",
            Piece::WhiteRook => "white/rook.png",
            Piece::WhiteQueen => "white/queen.png",
            Piece::WhiteKing => "white/king.png",
            Piece::BlackPawn => "black/pawn.png",
            Piece::BlackKnight => "black/knight.png",
            Piece::BlackBishop => "black/bishop.png",
            Piece::BlackRook => "black/rook.png",
            Piece::BlackQueen => "black/queen.png",
            Piece::BlackKing => "black/king.png",
            Piece::None => unreachable!("encountered Piece::None"),
        };

        let asset = asset_server.load(format!("pieces/{}", path));
        sprite_handler.add(*piece, asset);
    }

    cmd.insert_resource(sprite_handler);
}

fn spawn_pieces(
    mut cmd: Commands,
    sprite_handler: Res<PieceSpriteHandler>,
    board: Res<ChessBoard>,
) {
    let mut spawn_piece = |p: u8, piece_type: Piece| {
        let sprite = Sprite {
            image: sprite_handler.get(piece_type).cloned().expect("infallible"),
            custom_size: Some(Vec2::splat(PIECE_SIZE)),
            ..Default::default()
        };

        let layer = RenderLayer::Pieces;

        let board_pos = BoardPosition::new(SQ::from(p));

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
    app.add_systems(Startup, (init_handler, spawn_pieces).chain());
}
