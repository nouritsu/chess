use super::{common::get_moves, selector::SelectorState, Selector};
use crate::{
    app::RenderLayer,
    board::{BoardPosition, ChessBoard, CELL_SIZE},
    handler::{ColorHandler, GameColor, Handler},
};
use bevy::prelude::*;

#[derive(Resource)]
struct Highlighted;

#[derive(Component)]
struct HighlightMarker;

fn highlight_squares(
    mut cmd: Commands,
    selector: Res<Selector>,
    board: Res<ChessBoard>,
    color_handler: Res<ColorHandler>,
    mut meshes: ResMut<Assets<Mesh>>,
    hs: Option<Res<Highlighted>>,
) {
    if hs.is_some() {
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

    cmd.insert_resource(Highlighted);
}

fn remove_highlight(
    mut cmd: Commands,
    highlighted_squares: Query<Entity, With<HighlightMarker>>,
    selector: Res<Selector>,
    hs: Option<Res<Highlighted>>,
) {
    if hs.is_none() || selector.state == SelectorState::FromSelected {
        return;
    }

    for entity in highlighted_squares.iter() {
        cmd.entity(entity).despawn();
    }

    cmd.remove_resource::<Highlighted>();
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (remove_highlight, highlight_squares).chain());
}
