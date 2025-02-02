use crate::{
    app::{WINDOW_HEIGHT, WINDOW_WIDTH},
    board::{BOARD_SIZE, CELL_SIZE},
};
use bevy::prelude::*;
use map_range::MapRange;
use pleco::SQ;
use std::ops::Range;

#[derive(Component, Clone, Copy, Debug)]
#[require(Transform)]
pub struct BoardPosition(pub SQ);

impl BoardPosition {
    const BOARD_RANGE: Range<f32> = (0.)..((BOARD_SIZE - 1) as f32);

    const CELL_X_RANGE: Range<f32> = {
        let start = (-WINDOW_WIDTH + CELL_SIZE) / 2.;
        let end = (WINDOW_WIDTH - CELL_SIZE) / 2.;
        start..end
    };

    const CELL_Y_RANGE: Range<f32> = {
        let start = (-WINDOW_HEIGHT + CELL_SIZE) / 2.;
        let end = (WINDOW_HEIGHT - CELL_SIZE) / 2.;
        start..end
    };

    pub fn to_board_xy(&self) -> (u8, u8) {
        let p = u8::from(self.0);
        (p % 8, p / 8)
    }

    pub fn to_window_xy(&self) -> (f32, f32) {
        let (x, y) = self.to_board_xy();
        Self::board_to_window(x, y)
    }

    pub fn from_board_xy(x: u8, y: u8) -> Self {
        Self(SQ::from(x + y * BOARD_SIZE))
    }

    pub fn from_window_xy(x: f32, y: f32) -> Self {
        let (x, y) = Self::window_to_board(x, y);
        Self::from_board_xy(x, y)
    }

    fn board_to_window(x: u8, y: u8) -> (f32, f32) {
        (
            (x as f32).map_range(Self::BOARD_RANGE, Self::CELL_X_RANGE),
            (y as f32).map_range(Self::BOARD_RANGE, Self::CELL_Y_RANGE),
        )
    }

    fn window_to_board(x: f32, y: f32) -> (u8, u8) {
        (
            x.map_range(Self::CELL_X_RANGE, Self::BOARD_RANGE).round() as u8,
            y.map_range(Self::CELL_Y_RANGE, Self::BOARD_RANGE).round() as u8,
        )
    }
}

fn map_position(mut query: Query<(&BoardPosition, &mut Transform), Changed<BoardPosition>>) {
    for (pos, mut transform) in query.iter_mut() {
        let ((x, y), z) = (pos.to_window_xy(), transform.translation.x);
        *transform = Transform::from_xyz(x, y, z);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, map_position);
}
