use crate::{app::CursorPosition, board::BoardPosition, board::ChessBoard};
use bevy::prelude::*;
use pleco::SQ;

#[derive(Resource, Debug, Default)]
pub struct Selector {
    pub start: Option<SQ>,
    pub end: Option<SQ>,
    pub state: SelectorState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SelectorState {
    #[default]
    Idle,
    FromSelected,
    ToSelected,
}

fn init_selector(mut cmd: Commands) {
    let selector = Selector::default();
    cmd.insert_resource(selector);
}

fn selector(
    mut selector: ResMut<Selector>,
    cursor_position: Res<CursorPosition>,
    board: Res<ChessBoard>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let CursorPosition(Vec2 { x, y }) = *cursor_position;
    let BoardPosition(bp) = BoardPosition::from_window_xy(x, y);
    let pieces = board.get_piece_locations();

    match selector.state {
        SelectorState::Idle if buttons.just_released(MouseButton::Left) && pieces.at_square(bp) => {
            selector.start = Some(bp);
            selector.state = SelectorState::FromSelected;
        }

        SelectorState::FromSelected if buttons.just_released(MouseButton::Left) => {
            selector.end = Some(bp);
            selector.state = SelectorState::ToSelected;
        }

        SelectorState::ToSelected => {
            selector.start = None;
            selector.end = None;
            selector.state = SelectorState::Idle;
        }
        _ => {}
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, init_selector)
        .add_systems(Update, selector.run_if(resource_exists::<ChessBoard>));
}
