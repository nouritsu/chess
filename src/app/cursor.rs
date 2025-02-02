use super::{MainCamera, RenderLayer};
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource, Default)]
pub struct CursorPosition(Vec2);

#[derive(Component, Debug)]
pub enum CursorState {
    Default,
}

impl ToString for CursorState {
    fn to_string(&self) -> String {
        match self {
            CursorState::Default => "default".to_owned(),
        }
    }
}

impl Default for CursorState {
    fn default() -> Self {
        CursorState::Default
    }
}

impl CursorPosition {
    pub fn new(position: Vec2) -> Self {
        Self(position)
    }

    pub fn get(&self) -> Vec2 {
        self.0
    }

    pub fn set(&mut self, position: Vec2) {
        self.0 = position;
    }
}

fn init_cursor(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let transform = Transform::from_xyz(0., 0., RenderLayer::Cursor.into());

    let state = CursorState::default();

    let sprite = {
        let path = format!("cursor/{}.png", state.to_string());
        let image = asset_server.load(path);
        Sprite::from_image(image)
    };

    let layer = RenderLayer::Cursor;

    cmd.spawn((transform, sprite, state, layer));
    cmd.insert_resource(CursorPosition(Vec2::splat(0.)));
}

fn update_cursor_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let (camera, camera_transform) = camera.single();
    let window = windows.get_single().ok();

    let mapped_position = window
        .and_then(|window| window.cursor_position())
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .and_then(|ray| Some(ray.origin.truncate()));

    if let Some(position) = mapped_position {
        cursor_position.set(position);
    }
}

fn update_sprite_position(
    cursor_position: Res<CursorPosition>,
    mut cursor: Query<&mut Transform, With<CursorState>>,
) {
    if let Ok(mut transform) = cursor.get_single_mut() {
        transform.translation = cursor_position.get().extend(RenderLayer::Cursor.into());
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, init_cursor).add_systems(
        Update,
        (update_cursor_position, update_sprite_position).chain(),
    );
}
