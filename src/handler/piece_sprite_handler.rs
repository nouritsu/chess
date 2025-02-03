use super::{AsAssetPath, Handler};
use bevy::prelude::*;
use pleco::Piece;
use std::{collections::HashMap, path::Path};
use strum::VariantArray;

#[derive(Resource, Default)]
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

impl AsAssetPath for Piece {
    fn as_asset_path(&self) -> &'static Path {
        match self {
            Piece::WhitePawn => Path::new("pieces/white/pawn.png"),
            Piece::WhiteKnight => Path::new("pieces/white/knight.png"),
            Piece::WhiteBishop => Path::new("pieces/white/bishop.png"),
            Piece::WhiteRook => Path::new("pieces/white/rook.png"),
            Piece::WhiteQueen => Path::new("pieces/white/queen.png"),
            Piece::WhiteKing => Path::new("pieces/white/king.png"),
            Piece::BlackPawn => Path::new("pieces/black/pawn.png"),
            Piece::BlackKnight => Path::new("pieces/black/knight.png"),
            Piece::BlackBishop => Path::new("pieces/black/bishop.png"),
            Piece::BlackRook => Path::new("pieces/black/rook.png"),
            Piece::BlackQueen => Path::new("pieces/black/queen.png"),
            Piece::BlackKing => Path::new("pieces/black/king.png"),
            Piece::None => unimplemented!(),
        }
    }
}

fn init_sprite_handler(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let mut sprite_handler = PieceSpriteHandler::default();

    for piece in Piece::VARIANTS.iter().filter(|&&p| p != Piece::None) {
        let asset = asset_server.load(piece.as_asset_path());
        sprite_handler.add(*piece, asset);
    }

    cmd.insert_resource(sprite_handler);
}

pub fn plugin(app: &mut App) {
    app.add_systems(PreStartup, init_sprite_handler);
}
