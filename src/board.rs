use crate::{
    app::{WINDOW_HEIGHT, WINDOW_WIDTH},
    board_position::BoardPosition,
    handler::{ColorHandler, GameColor, Handler},
    render_layer::RenderLayer,
};
use bevy::prelude::*;
use pleco::SQ;

pub const CELL_SIZE: f32 = 64.;

pub const BOARD_SIZE: usize = {
    let board_size = 8.;

    assert!(board_size % 1. == 0., "board size must be an integer");
    assert!(
        WINDOW_WIDTH % (board_size * CELL_SIZE) == 0.,
        "window width must be divisible by board size"
    );
    assert!(
        WINDOW_HEIGHT % (board_size * CELL_SIZE) == 0.,
        "window height must be divisible by board size"
    );

    board_size as usize
};

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
    for p in 0..(BOARD_SIZE * BOARD_SIZE) {
        let mesh = {
            let rec = Rectangle::new(CELL_SIZE, CELL_SIZE);
            Mesh2d(meshes.add(Mesh::from(rec)))
        };

        let board_position = BoardPosition::new(SQ::from(p as u8));

        let material = {
            let color = color_handler
                .get(GameColor::from((p % 8, p / 8)))
                .cloned()
                .expect("infallible");

            MeshMaterial2d(color)
        };

        let layer = RenderLayer::Board;

        cmd.spawn((mesh, material, board_position, layer));
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, (init_handler, init_board).chain());
}
