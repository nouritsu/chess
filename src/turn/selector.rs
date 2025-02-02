use crate::{app::CursorPosition, board::BoardPosition, board::ChessBoard};
use bevy::prelude::*;
use pleco::SQ;

use super::HighlightState;

#[derive(Resource, Debug)]
pub struct Selector {
    start: Option<SQ>,
    end: Option<SQ>,
    state: SelectorState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectorState {
    Idle,
    FromSelected,
    ToSelected,
}

#[derive(Event)]
pub struct SelectorFromSelected;

impl Selector {
    pub fn new() -> Self {
        Selector {
            start: None,
            end: None,
            state: SelectorState::Idle,
        }
    }

    pub fn start(&mut self, position: SQ) {
        self.start = Some(position);
        self.state = SelectorState::FromSelected;
    }

    pub fn get_from(&self) -> Option<SQ> {
        self.start
    }

    pub fn end(&mut self, position: SQ) {
        self.end = Some(position);
        self.state = SelectorState::ToSelected;
    }

    pub fn get_to(&self) -> Option<SQ> {
        self.end
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.end = None;
        self.state = SelectorState::Idle;
    }

    pub fn state(&self) -> SelectorState {
        self.state
    }
}

fn init_selector(mut cmd: Commands) {
    let selector = Selector::new();
    cmd.insert_resource(selector);
}

fn selector(
    mut selector: ResMut<Selector>,
    cursor_position: Res<CursorPosition>,
    board: Res<ChessBoard>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    let Vec2 { x, y } = cursor_position.get();
    let bp = BoardPosition::from_window_xy(x, y).to_sq();
    let pieces = board.get_piece_locations();

    match selector.state() {
        SelectorState::Idle if buttons.just_released(MouseButton::Left) && pieces.at_square(bp) => {
            selector.start(bp);
        }

        SelectorState::FromSelected if buttons.just_released(MouseButton::Left) => {
            selector.end(bp);
        }

        SelectorState::ToSelected => {
            selector.reset();
        }
        _ => {}
    }
}

fn highlight_state_handler(selector: Res<Selector>, mut hs: ResMut<HighlightState>) {
    if matches!(selector.state(), SelectorState::FromSelected)
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
