use super::BoardPosition;
use crate::{
    app::RenderLayer,
    handler::{ColorHandler, GameColor, Handler},
};
use bevy::prelude::*;
use pleco::{Board, SQ};
use std::ops::{Deref, DerefMut};

pub const CELL_SIZE: f32 = 64.;
pub const BOARD_SIZE: u8 = 8;

#[derive(Resource)]
pub struct ChessBoard(Board);

impl Deref for ChessBoard {
    type Target = Board;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChessBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn init_board(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    color_handler: Res<ColorHandler>,
) {
    for p in (0..(BOARD_SIZE * BOARD_SIZE)).map(|x| SQ::from(x)) {
        let mesh = {
            let rec = Rectangle::new(CELL_SIZE, CELL_SIZE);
            Mesh2d(meshes.add(Mesh::from(rec)))
        };

        let board_position = BoardPosition(p);

        let material = {
            let color = color_handler.get(GameColor::from(p));
            MeshMaterial2d(color)
        };

        let layer = RenderLayer::Board;

        cmd.spawn((mesh, material, board_position, layer));
    }
}

pub fn plugin(app: &mut App) {
    let board = ChessBoard(Board::default());
    app.insert_resource(board);
    app.add_systems(Startup, init_board);
}
