use super::{common::get_moves, selector::SelectorState, Selector};
use crate::{
    app::RenderLayer,
    board::{BoardPosition, ChessBoard, CELL_SIZE},
    handler::{ColorHandler, GameColor, Handler},
};
use bevy::prelude::*;

#[derive(Component)]
struct HighlightMarker;

#[derive(Resource, Debug, PartialEq, Eq)]
pub enum HighlightState {
    Idle,
    FromSelected,
    Spawned,
}

fn init_highlight_state(mut cmd: Commands) {
    cmd.insert_resource(HighlightState::Idle);
}

fn highlight_squares(
    mut cmd: Commands,
    selector: Res<Selector>,
    board: Res<ChessBoard>,
    color_handler: Res<ColorHandler>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut hs: ResMut<HighlightState>,
) {
    if matches!(hs.as_ref(), HighlightState::Spawned) {
        return;
    }

    let Some(sq) = selector.start else {
        return;
    };

    let piece = board.get_piece_locations().piece_at(sq);

    let squares = get_moves(&board, sq, piece);

    for square in squares {
        let mesh = {
            let rec = Rectangle::new(CELL_SIZE, CELL_SIZE);
            Mesh2d(meshes.add(Mesh::from(rec)))
        };

        let board_position = BoardPosition(square);

        let material = {
            let color = color_handler.get(GameColor::HighlightGreen);
            MeshMaterial2d(color)
        };

        let layer = RenderLayer::Highlight;

        let marker = HighlightMarker;

        cmd.spawn((mesh, material, board_position, layer, marker));
    }

    *hs = HighlightState::Spawned;
}

fn remove_highlight(
    mut cmd: Commands,
    highlighted_squares: Query<Entity, With<HighlightMarker>>,
    selector: Res<Selector>,
    mut hs: ResMut<HighlightState>,
) {
    if matches!(selector.state, SelectorState::FromSelected)
        || matches!(*hs, HighlightState::FromSelected | HighlightState::Idle)
    {
        return;
    }

    for entity in highlighted_squares.iter() {
        cmd.entity(entity).despawn();
    }

    *hs = HighlightState::Idle;
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, init_highlight_state)
        .add_systems(Update, (remove_highlight, highlight_squares).chain());
}
