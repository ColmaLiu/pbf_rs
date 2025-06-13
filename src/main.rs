use bevy::prelude::*;

use crate::set_scene::{camera_control_system, setup, simulation_step};
use crate::simulator::Simulator;
mod parallel;
mod set_scene;
mod simulator;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Simulator::new())
        .add_systems(Startup, setup)
        .add_systems(Update, camera_control_system)
        .add_systems(Update, simulation_step)
        .run();
}
