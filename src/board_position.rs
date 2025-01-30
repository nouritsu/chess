use crate::{
    app::{WINDOW_HEIGHT, WINDOW_WIDTH},
    board::{BOARD_SIZE, CELL_SIZE},
};
use bevy::prelude::*;
use map_range::MapRange;
use pleco::SQ;

#[derive(Component, Clone, Copy)]
#[require(Transform)]
pub struct BoardPosition(SQ);

impl From<BoardPosition> for Vec2 {
    fn from(pos: BoardPosition) -> Self {
        let p = u8::from(pos.0);

        let x_input_range = {
            let start = 0.;
            let end = (BOARD_SIZE - 1) as f32;
            start..end
        };

        let y_input_range = {
            let start = 0.;
            let end = (BOARD_SIZE - 1) as f32;
            start..end
        };

        let x_output_range = {
            let start = (-WINDOW_WIDTH + CELL_SIZE) / 2.;
            let end = (WINDOW_WIDTH - CELL_SIZE) / 2.;
            start..end
        };

        let y_output_range = {
            let start = (-WINDOW_HEIGHT + CELL_SIZE) / 2.;
            let end = (WINDOW_HEIGHT - CELL_SIZE) / 2.;
            start..end
        };

        let x = ((p % 8) as f32).map_range(x_input_range, x_output_range);
        let y = ((p / 8) as f32).map_range(y_input_range, y_output_range);

        Vec2::new(x, y)
    }
}

impl BoardPosition {
    pub fn new(sq: SQ) -> Self {
        Self(sq)
    }
}

fn map_position(mut query: Query<(&BoardPosition, &mut Transform), Changed<BoardPosition>>) {
    for (pos, mut transform) in query.iter_mut() {
        let (Vec2 { x, y }, z) = (Vec2::from(*pos), transform.translation.x);
        *transform = Transform::from_xyz(x, y, z);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, map_position);
}
