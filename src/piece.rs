use crate::{
    board_position::BoardPosition,
    handler::{GameColor, GamePiece, Handler, PieceSpriteHandler},
    render_layer::RenderLayer,
};
use bevy::prelude::*;
use itertools::iproduct;
use pleco::SQ;
use strum::VariantArray;

fn init_handler(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let mut sprite_handler = PieceSpriteHandler::new();

    for (&color, &piece) in iproduct!(GameColor::VARIANTS, GamePiece::VARIANTS) {
        let path = format!("pieces/{}/{}.png", color.to_string(), piece.to_string());
        let asset = asset_server.load(path);
        sprite_handler.add((color, piece), asset);
    }

    cmd.insert_resource(sprite_handler);
}

fn spawn_pieces(mut cmd: Commands, sprite_handler: Res<PieceSpriteHandler>) {
    let mut spawn_piece = |p: u8, piece_color: GameColor, piece_type: GamePiece| {
        let sprite = Sprite {
            image: sprite_handler
                .get((piece_color, piece_type))
                .cloned()
                .expect("infallible"),
            custom_size: Some(Vec2::splat(48.)),
            ..Default::default()
        };

        let layer = RenderLayer::Pieces;

        let board_pos = BoardPosition::new(SQ::from(p));

        cmd.spawn((board_pos, sprite, layer));
    };

    // Kings
    spawn_piece(4, GameColor::White, GamePiece::King);
    spawn_piece(60, GameColor::Black, GamePiece::King);

    // Queens
    spawn_piece(3, GameColor::White, GamePiece::Queen);
    spawn_piece(59, GameColor::Black, GamePiece::Queen);

    // Rooks
    spawn_piece(0, GameColor::White, GamePiece::Rook);
    spawn_piece(7, GameColor::White, GamePiece::Rook);
    spawn_piece(56, GameColor::Black, GamePiece::Rook);
    spawn_piece(63, GameColor::Black, GamePiece::Rook);

    // Bishops
    spawn_piece(2, GameColor::White, GamePiece::Bishop);
    spawn_piece(5, GameColor::White, GamePiece::Bishop);
    spawn_piece(58, GameColor::Black, GamePiece::Bishop);
    spawn_piece(61, GameColor::Black, GamePiece::Bishop);

    // Knights
    spawn_piece(1, GameColor::White, GamePiece::Knight);
    spawn_piece(6, GameColor::White, GamePiece::Knight);
    spawn_piece(57, GameColor::Black, GamePiece::Knight);
    spawn_piece(62, GameColor::Black, GamePiece::Knight);

    // Pawns
    for i in 8..16 {
        spawn_piece(i, GameColor::White, GamePiece::Pawn);
    }

    for i in 48..56 {
        spawn_piece(i, GameColor::Black, GamePiece::Pawn);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (init_handler, spawn_pieces).chain());
}
