use super::Handler;
use bevy::prelude::*;
use pleco::Piece;
use std::collections::HashMap;

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
