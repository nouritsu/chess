use bevy::prelude::*;

pub const IDX_WHITE: usize = 0;
pub const IDX_BLACK: usize = 1;

#[derive(Resource)]
pub struct ColorHandler(Vec<Handle<ColorMaterial>>);

impl ColorHandler {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, color: Handle<ColorMaterial>) {
        self.0.push(color);
    }

    pub fn get(&self, index: usize) -> &Handle<ColorMaterial> {
        &self.0[index]
    }
}
