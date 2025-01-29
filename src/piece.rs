use bevy::prelude::*;

pub const SPRITE_SIZE: f32 = 64.;

#[derive(Component)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

fn setup(mut cmd: Commands) {}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
