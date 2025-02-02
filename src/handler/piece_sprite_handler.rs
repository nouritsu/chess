use super::Handler;
use bevy::prelude::*;
use pleco::Piece;
use std::collections::HashMap;
use strum::VariantArray;

#[derive(Resource)]
pub struct PieceSpriteHandler(HashMap<Piece, Handle<Image>>);

impl Handler for PieceSpriteHandler {
    type K = Piece;
    type A = Image;

    fn get_inner(&self) -> &HashMap<Self::K, Handle<Self::A>> {
        &self.0
    }

    fn get_inner_mut(&mut self) -> &mut HashMap<Self::K, Handle<Self::A>> {
        &mut self.0
    }
}

impl PieceSpriteHandler {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

fn init_sprite_handler(mut cmd: Commands, asset_server: Res<AssetServer>) {
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

pub fn plugin(app: &mut App) {
    app.add_systems(PreStartup, init_sprite_handler);
}
