use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
#[require(Transform)]
pub enum RenderLayer {
    Board,
    Pieces,
    Cursor,
}

impl From<RenderLayer> for f32 {
    fn from(layer: RenderLayer) -> Self {
        match layer {
            RenderLayer::Board => 0.,
            RenderLayer::Pieces => 1.,
            RenderLayer::Cursor => 2.,
        }
    }
}

fn correct_layers(mut query: Query<(&RenderLayer, &mut Transform), Changed<RenderLayer>>) {
    for (&layer, mut transform) in query.iter_mut() {
        transform.translation.z = layer.into();
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, correct_layers);
}
