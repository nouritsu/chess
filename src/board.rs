use crate::{
    app::{WINDOW_HEIGHT, WINDOW_WIDTH},
    board_position::BoardPosition,
    colors::{ColorHandler, IDX_BLACK, IDX_WHITE},
};
use bevy::prelude::*;
use itertools::iproduct;

pub const BOARD_SIZE: usize = {
    let board_size = 8.;
    assert!(board_size % 1. == 0.);
    assert!(WINDOW_WIDTH % (board_size * CELL_SIZE) == 0.);
    assert!(WINDOW_HEIGHT % (board_size * CELL_SIZE) == 0.);
    board_size as usize
};

pub const CELL_SIZE: f32 = 64.;

const BOARD_WHITE: &'static str = "#E6EAD7";
const BOARD_BLACK: &'static str = "#454D5F";

fn setup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    let mut colors_handler = ColorHandler::new();

    let black: Color = Srgba::hex(BOARD_BLACK).unwrap().into();
    let white: Color = Srgba::hex(BOARD_WHITE).unwrap().into();

    colors_handler.add(colors.add(black));
    colors_handler.add(colors.add(white));

    for (i, j) in iproduct!(0..BOARD_SIZE, 0..BOARD_SIZE) {
        let mesh = {
            let rec = Rectangle::new(CELL_SIZE, CELL_SIZE);
            Mesh2d(meshes.add(Mesh::from(rec)))
        };

        let board_position = BoardPosition::new(i, j);

        let material = {
            let color = if (i + j) % 2 == 0 {
                IDX_WHITE
            } else {
                IDX_BLACK
            };

            MeshMaterial2d(colors_handler.get(color).clone())
        };

        cmd.spawn((mesh, material, board_position));
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}
