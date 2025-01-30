use super::Handler;
use bevy::prelude::*;
use std::collections::HashMap;
use strum::VariantArray;

#[derive(Resource)]
pub struct ColorHandler(HashMap<GameColor, Handle<ColorMaterial>>);

#[derive(Component, Hash, Eq, PartialEq, Clone, Copy, VariantArray)]
pub enum GameColor {
    BoardWhite,
    BoardBlack,
}

impl ToString for GameColor {
    fn to_string(&self) -> String {
        match self {
            Self::BoardWhite => "white",
            Self::BoardBlack => "black",
        }
        .to_string()
    }
}

impl From<GameColor> for Srgba {
    fn from(color: GameColor) -> Self {
        Self::hex(color.hex()).expect("unable to convert hex")
    }
}

impl From<(usize, usize)> for GameColor {
    fn from((i, j): (usize, usize)) -> Self {
        if (i + j) % 2 == 0 {
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

impl ColorHandler {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
