use crate::{
    board_position::BoardPosition,
    handler::{ColorHandler, GameColor, Handler},
    render_layer::RenderLayer,
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

fn init_handler(mut cmd: Commands, mut colors: ResMut<Assets<ColorMaterial>>) {
    let mut color_handler = ColorHandler::new();
    let black: Color = Srgba::from(GameColor::BoardBlack).into();
    let white: Color = Srgba::from(GameColor::BoardWhite).into();

    color_handler.add(GameColor::BoardBlack, colors.add(black));
    color_handler.add(GameColor::BoardWhite, colors.add(white));

    cmd.insert_resource(color_handler);
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

        let board_position = BoardPosition::new(p);

        let material = {
            let color = color_handler
                .get(GameColor::from(p))
                .cloned()
                .expect("failed to get color");

            MeshMaterial2d(color)
        };

        let layer = RenderLayer::Board;

        cmd.spawn((mesh, material, board_position, layer));
    }
}

pub fn plugin(app: &mut App) {
    let board = ChessBoard(Board::default());
    app.insert_resource(board);
    app.add_systems(Startup, (init_handler, init_board).chain());
}
