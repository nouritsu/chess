use crate::app::MainCamera;
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub enum CursorState {
    Default,
}

impl Default for CursorState {
    fn default() -> Self {
        CursorState::Default
    }
}

fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let transform = Transform::from_xyz(0., 0., 0.);

    let state = CursorState::default();

    let sprite = {
        let image = asset_server.load("cursor-default.png");
        Sprite::from_image(image)
    };

    cmd.spawn((transform, sprite, state));
}

fn update(
    mut cursor_info: Query<(&mut Transform, &CursorState)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera.single();
    let (mut c_position, _c_state) = cursor_info.single_mut();
    let m_position = windows.get_single().map(|window| window.cursor_position());

    if let Ok(Some(m_position)) = m_position {
        if let Ok(mapped_pos) = camera.viewport_to_world(camera_transform, m_position) {
            c_position.translation = mapped_pos.origin.xy().extend(0.);
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup).add_systems(Update, update);
}
