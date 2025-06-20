use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

use crate::scene::{
    camera_control_system, pause_resume_button_system, reset_sim_button_system, scene_refresh_system, setup, simulation_step, switch_scene_button_system, update_boundary
};
use crate::simulator::Simulator;

mod scene;
mod simulator;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        font_size: 20.0,
                        font: default(),
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },
                    text_color: Color::srgb(0.0, 1.0, 0.0),
                    refresh_interval: core::time::Duration::from_millis(100),
                    enabled: true,
                },
            },
        ))
        .insert_resource(Simulator::new())
        .insert_resource(scene::SimRunning(true))
        .add_systems(Startup, setup)
        .add_systems(Update, camera_control_system)
        .add_systems(Update, pause_resume_button_system)
        .add_systems(Update, switch_scene_button_system)
        .add_systems(Update, reset_sim_button_system)
        .add_systems(Update, scene_refresh_system)
        .add_systems(Update, update_boundary)
        .add_systems(PostUpdate, simulation_step)
        .run();
}
