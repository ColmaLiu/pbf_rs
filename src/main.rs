use bevy::prelude::*;

use crate::set_scene::setup;

mod set_scene;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
