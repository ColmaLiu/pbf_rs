use bevy::prelude::*;

use crate::set_scene::{camera_control_system, setup, simulation_step, sync_particles_system};
use crate::simulator::Simulator;
mod set_scene;
mod simulator;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Simulator::new())
        .add_systems(Startup, setup)
        // 在每一帧更新时都运行我们的摄像头控制系统
        .add_systems(
            Update,
            (
                camera_control_system,
                // sync_particles_system,
                // simulation_step,
            ),
        )
        .run();
}
