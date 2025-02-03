use super::BoardPosition;
use crate::{
    app::RenderLayer,
    handler::{ColorHandler, GameColor, Handler},
};
use bevy::prelude::*;
use derive_more::derive::{Deref, DerefMut};
use pleco::{Board, SQ};

pub const CELL_SIZE: f32 = 64.;
pub const BOARD_SIZE: u8 = 8;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ChessBoard(Board);

fn init_board(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    color_handler: Res<ColorHandler>,
) {
    let board = ChessBoard::default();

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

    cmd.insert_resource(board);
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, init_board);
}
