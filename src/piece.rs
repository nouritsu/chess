use crate::{
    board_position::BoardPosition,
    handler::{GameColor, GamePiece, Handler, PieceSpriteHandler},
    render_layer::RenderLayer,
};
use bevy::prelude::*;
use itertools::iproduct;
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
    let mut spawn_piece =
        |(i, j): (usize, usize), piece_color: GameColor, piece_type: GamePiece| {
            let sprite = Sprite {
                image: sprite_handler
                    .get((piece_color, piece_type))
                    .cloned()
                    .expect("infallible"),
                custom_size: Some(Vec2::splat(48.)),
                ..Default::default()
            };

            let layer = RenderLayer::Pieces;

            let board_pos = BoardPosition::new(i, j);

            cmd.spawn((board_pos, sprite, layer));
        };

    // Kings
    spawn_piece((4, 0), GameColor::White, GamePiece::King);
    spawn_piece((4, 7), GameColor::Black, GamePiece::King);

    // Queens
    spawn_piece((3, 0), GameColor::White, GamePiece::Queen);
    spawn_piece((3, 7), GameColor::Black, GamePiece::Queen);

    // Rooks
    spawn_piece((7, 0), GameColor::White, GamePiece::Rook);
    spawn_piece((7, 7), GameColor::Black, GamePiece::Rook);
    spawn_piece((0, 0), GameColor::White, GamePiece::Rook);
    spawn_piece((0, 7), GameColor::Black, GamePiece::Rook);

    // Bishops
    spawn_piece((2, 0), GameColor::White, GamePiece::Bishop);
    spawn_piece((5, 0), GameColor::White, GamePiece::Bishop);
    spawn_piece((2, 7), GameColor::Black, GamePiece::Bishop);
    spawn_piece((5, 7), GameColor::Black, GamePiece::Bishop);

    // Knights
    spawn_piece((1, 0), GameColor::White, GamePiece::Knight);
    spawn_piece((6, 0), GameColor::White, GamePiece::Knight);
    spawn_piece((1, 7), GameColor::Black, GamePiece::Knight);
    spawn_piece((6, 7), GameColor::Black, GamePiece::Knight);

    // Pawns
    for i in 0..8 {
        spawn_piece((i, 1), GameColor::White, GamePiece::Pawn);
        spawn_piece((i, 6), GameColor::Black, GamePiece::Pawn);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (init_handler, spawn_pieces).chain());
}
