use crate::{app::CursorPosition, board::BoardPosition, board::ChessBoard};
use bevy::prelude::*;
use pleco::SQ;

use super::HighlightState;

#[derive(Resource, Debug, Default)]
pub struct Selector {
    pub start: Option<SQ>,
    pub end: Option<SQ>,
    pub state: SelectorState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectorState {
    Idle,
    FromSelected,
    ToSelected,
}

impl Default for SelectorState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Event)]
pub struct SelectorFromSelected;

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

fn highlight_state_handler(selector: Res<Selector>, mut hs: ResMut<HighlightState>) {
    if matches!(selector.state, SelectorState::FromSelected)
        && !matches!(hs.as_ref(), HighlightState::Spawned)
    {
        *hs = HighlightState::FromSelected;
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(PreStartup, init_selector)
        .add_systems(Update, (selector, highlight_state_handler).chain())
        .add_event::<SelectorFromSelected>();
}
