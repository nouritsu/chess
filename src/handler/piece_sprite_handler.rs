use super::{GameColor, Handler};
use bevy::prelude::*;
use std::collections::HashMap;
use strum::VariantArray;

#[derive(Resource)]
pub struct PieceSpriteHandler(HashMap<(GameColor, GamePiece), Handle<Image>>);

#[derive(Component, Hash, Eq, PartialEq, Clone, Copy, VariantArray)]
pub enum GamePiece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl ToString for GamePiece {
    fn to_string(&self) -> String {
        match self {
            Self::Pawn => "pawn",
            Self::Rook => "rook",
            Self::Knight => "knight",
            Self::Bishop => "bishop",
            Self::Queen => "queen",
            Self::King => "king",
        }
        .to_string()
    }
}

impl Handler for PieceSpriteHandler {
    type K = (GameColor, GamePiece);
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
