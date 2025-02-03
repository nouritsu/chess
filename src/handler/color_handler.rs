use super::Handler;
use bevy::prelude::*;
use pleco::SQ;
use std::collections::HashMap;
use strum::VariantArray;

#[derive(Resource, Default)]
pub struct ColorHandler(HashMap<GameColor, Handle<ColorMaterial>>);

#[derive(Component, Hash, Eq, PartialEq, Clone, Copy, VariantArray)]
pub enum GameColor {
    BoardWhite,
    BoardBlack,
    HighlightGreen,
}

impl From<GameColor> for Srgba {
    fn from(color: GameColor) -> Self {
        Self::hex(color.hex()).expect("failed to build srgba from hex")
    }
}

impl From<SQ> for GameColor {
    fn from(sq: SQ) -> Self {
        let rank = sq.rank_idx_of_sq();
        let file = sq.file_idx_of_sq();

        if (rank + file) % 2 == 0 {
            Self::BoardBlack
        } else {
            Self::BoardWhite
        }
    }
}

impl GameColor {
    pub fn hex(&self) -> &'static str {
        match self {
            Self::BoardWhite => "#E6EAD7",
            Self::BoardBlack => "#454D5F",
            Self::HighlightGreen => "#00FF00",
        }
    }
}

impl Handler for ColorHandler {
    type K = GameColor;
    type A = ColorMaterial;

    fn get_inner(&self) -> &HashMap<Self::K, Handle<Self::A>> {
        &self.0
    }

    fn get_inner_mut(&mut self) -> &mut HashMap<Self::K, Handle<Self::A>> {
        &mut self.0
    }
}

fn init_color_handler(mut cmd: Commands, mut colors: ResMut<Assets<ColorMaterial>>) {
    let mut color_handler = ColorHandler::default();

    for &color in GameColor::VARIANTS {
        let srgba: Srgba = color.into();
        color_handler.add(color, colors.add(Color::from(srgba)));
    }

    cmd.insert_resource(color_handler);
}

pub fn plugin(app: &mut App) {
    app.add_systems(PreStartup, init_color_handler);
}
