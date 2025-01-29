use crate::{board, board_position, cursor};
use bevy::{prelude::*, window::CursorOptions};

pub const WINDOW_WIDTH: f32 = 512.;
pub const WINDOW_HEIGHT: f32 = 512.;

#[derive(Component)]
pub struct MainCamera;

fn init_camera(mut cmd: Commands) {
    cmd.spawn((Camera2d::default(), MainCamera));
}

pub fn plugin(app: &mut App) {
    let window = Window {
        title: "Chess".to_owned(),
        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
        cursor_options: CursorOptions {
            visible: false,
            ..Default::default()
        },
        ..default()
    };

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    });

    app.add_plugins((
        default_plugins,
        board::plugin,
        cursor::plugin,
        board_position::plugin,
    ))
    .add_systems(Startup, init_camera);
}
