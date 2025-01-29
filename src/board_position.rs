use crate::{
    app::{WINDOW_HEIGHT, WINDOW_WIDTH},
    board::{BOARD_SIZE, CELL_SIZE},
};
use bevy::prelude::*;
use map_range::MapRange;

#[derive(Component, Clone)]
#[require(Transform)]
pub struct BoardPosition(usize, usize);

impl BoardPosition {
    pub fn new(x: usize, y: usize) -> Self {
        BoardPosition(x, y)
    }

    pub fn x(&self) -> usize {
        self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }

    pub fn set_x(&mut self, x: usize) {
        self.0 = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.1 = y;
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.set_x(x);
        self.set_y(y);
    }
}

impl From<BoardPosition> for Transform {
    fn from(pos: BoardPosition) -> Self {
        let x_input_start = 0.;
        let y_input_start = 0.;

        let x_input_end = (BOARD_SIZE - 1) as f32;
        let y_input_end = (BOARD_SIZE - 1) as f32;

        let x_output_start = (-WINDOW_WIDTH + CELL_SIZE) / 2.;
        let y_output_start = (-WINDOW_HEIGHT + CELL_SIZE) / 2.;

        let x_output_end = (WINDOW_WIDTH - CELL_SIZE) / 2.;
        let y_output_end = (WINDOW_HEIGHT - CELL_SIZE) / 2.;

        let x =
            (pos.x() as f32).map_range(x_input_start..x_input_end, x_output_start..x_output_end);
        let y =
            (pos.y() as f32).map_range(y_input_start..y_input_end, y_output_start..y_output_end);

        Transform::from_xyz(x, y, 0.)
    }
}

fn map_position(mut query: Query<(&BoardPosition, &mut Transform), Changed<BoardPosition>>) {
    for (pos, mut transform) in query.iter_mut() {
        *transform = pos.clone().into();
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, map_position);
}
