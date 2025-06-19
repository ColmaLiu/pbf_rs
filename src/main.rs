use bevy::prelude::*;

use crate::set_scene::{
    camera_control_system, pause_resume_button_system, scene_refresh_system, setup,
    simulation_step, switch_scene_button_system, update_tank_box_system,
};
use crate::simulator::Simulator;
// mod parallel;
mod set_scene;
mod simulator;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Simulator::new())
        .insert_resource(set_scene::SimRunning(true))
        .add_systems(Startup, setup)
        .add_systems(Update, camera_control_system)
        .add_systems(Update, simulation_step)
        .add_systems(Update, pause_resume_button_system)
        .add_systems(Update, switch_scene_button_system)
        .add_systems(Update, scene_refresh_system)
        .add_systems(Update, update_tank_box_system)
        .run();
}
