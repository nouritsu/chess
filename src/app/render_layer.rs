use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
#[require(Transform)]
pub enum RenderLayer {
    Board,
    Highlight,
    Pieces,
    Cursor,
}

impl From<RenderLayer> for f32 {
    fn from(layer: RenderLayer) -> Self {
        match layer {
            RenderLayer::Board => 0.,
            RenderLayer::Highlight => 1.,
            RenderLayer::Pieces => 2.,
            RenderLayer::Cursor => 3.,
        }
    }
}

fn correct_layers(mut query: Query<(&RenderLayer, &mut Transform), Changed<RenderLayer>>) {
    for (&layer, mut transform) in query.iter_mut() {
        let Vec3 { x, y, z: _ } = transform.translation;
        transform.translation = Vec3::new(x, y, layer.into());
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, correct_layers);
}
