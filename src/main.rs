use bevy::prelude::*;
use chess::app::AppPlugin;

fn main() {
    App::new().add_plugins(AppPlugin).run();
}
