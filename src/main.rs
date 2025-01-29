use bevy::prelude::*;
use chess::app;

fn main() {
    App::new().add_plugins(app::plugin).run();
}
