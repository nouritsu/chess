use bevy::{prelude::*, window::CursorOptions};

pub const WINDOW_WIDTH: f32 = 512.;
pub const WINDOW_HEIGHT: f32 = 512.;

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

    let default_plugins = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        })
        .set(ImagePlugin::default_nearest());

    app.add_plugins(default_plugins);
}
