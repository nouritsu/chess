use crate::{app::MainCamera, render_layer::RenderLayer};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub enum CursorState {
    Default,
}

impl ToString for CursorState {
    fn to_string(&self) -> String {
        match self {
            CursorState::Default => "default".to_string(),
        }
    }
}

impl Default for CursorState {
    fn default() -> Self {
        CursorState::Default
    }
}

fn init_cursor(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let transform = Transform::from_xyz(0., 0., 0.);

    let state = CursorState::default();

    let sprite = {
        let path = format!("cursor/{}.png", state.to_string());
        let image = asset_server.load(path);
        Sprite::from_image(image)
    };

    let layer = RenderLayer::Cursor;

    cmd.spawn((transform, sprite, state, layer));
}

//TODO: this needs to be more readable
fn update_cursor_position(
    mut cursor_info: Query<(&mut Transform, &CursorState)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera.single();

    if let Ok(Some(m_position)) = windows.get_single().map(|window| window.cursor_position()) {
        if let (Ok(mapped_pos), Ok((mut cursor_position, _))) = (
            camera.viewport_to_world(camera_transform, m_position),
            cursor_info.get_single_mut(),
        ) {
            cursor_position.translation = mapped_pos.origin.xy().extend(0.);
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, init_cursor)
        .add_systems(Update, update_cursor_position);
}
