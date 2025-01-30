use crate::{
    board_position::BoardPosition,
    handler::{Handler, PieceSpriteHandler},
    render_layer::RenderLayer,
};
use bevy::prelude::*;
use pleco::{Piece, SQ};
use strum::VariantArray;

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

fn spawn_pieces(mut cmd: Commands, sprite_handler: Res<PieceSpriteHandler>) {
    let mut spawn_piece = |p: u8, piece_type: Piece| {
        let sprite = Sprite {
            image: sprite_handler.get(piece_type).cloned().expect("infallible"),
            custom_size: Some(Vec2::splat(48.)),
            ..Default::default()
        };

        let layer = RenderLayer::Pieces;

        let board_pos = BoardPosition::new(SQ::from(p));

        cmd.spawn((board_pos, sprite, layer));
    };

    // Kings
    spawn_piece(4, Piece::WhiteKing);
    spawn_piece(60, Piece::BlackKing);

    // Queens
    spawn_piece(3, Piece::WhiteQueen);
    spawn_piece(59, Piece::BlackQueen);

    // Rooks
    spawn_piece(0, Piece::WhiteRook);
    spawn_piece(7, Piece::WhiteRook);
    spawn_piece(56, Piece::BlackRook);
    spawn_piece(63, Piece::BlackRook);

    // Bishops
    spawn_piece(2, Piece::WhiteBishop);
    spawn_piece(5, Piece::WhiteBishop);
    spawn_piece(58, Piece::BlackBishop);
    spawn_piece(61, Piece::BlackBishop);

    // Knights
    spawn_piece(1, Piece::WhiteKnight);
    spawn_piece(6, Piece::WhiteKnight);
    spawn_piece(57, Piece::BlackKnight);
    spawn_piece(62, Piece::BlackKnight);

    // Pawns
    for i in 8..16 {
        spawn_piece(i, Piece::WhitePawn);
    }

    for i in 48..56 {
        spawn_piece(i, Piece::BlackPawn);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (init_handler, spawn_pieces).chain());
}
