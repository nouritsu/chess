use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

fn init_camera(mut cmd: Commands) {
    cmd.spawn((Camera2d::default(), MainCamera, Msaa::Off));
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, init_camera);
}
