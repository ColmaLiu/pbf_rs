use std::iter::zip;

use crate::simulator::Simulator;
use bevy::input::mouse::MouseWheel;

use bevy::{color::palettes::basic::*, prelude::*, render::render_asset::RenderAssetUsages};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Resource, Default)]
pub struct SimRunning(pub bool);

#[derive(Component)]
pub struct SwitchSceneButton;

#[derive(Component)]
pub struct PauseResumeButton;

#[derive(Component)]
pub struct ResetSimButton;

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Boundary;

fn button(asset_server: &AssetServer) -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Button,
                PauseResumeButton,
                Node {
                    width: Val::Px(220.0),
                    height: Val::Px(80.0),
                    border: UiRect::all(Val::Px(5.0)),
                    margin: UiRect {
                        left: Val::Px(20.0),
                        top: Val::Px(20.0),
                        ..default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                children![(
                    Text::new("Stop Simulation"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextShadow::default(),
                )]
            ),
            (
                Button,
                SwitchSceneButton,
                Node {
                    width: Val::Px(220.0),
                    height: Val::Px(80.0),
                    border: UiRect::all(Val::Px(5.0)),
                    margin: UiRect {
                        left: Val::Px(20.0),
                        top: Val::Px(20.0),
                        ..default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                children![(
                    Text::new("Switch Scene"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextShadow::default(),
                )]
            ),
            (
                Button,
                ResetSimButton,
                Node {
                    width: Val::Px(220.0),
                    height: Val::Px(80.0),
                    border: UiRect::all(Val::Px(5.0)),
                    margin: UiRect {
                        left: Val::Px(20.0),
                        top: Val::Px(20.0),
                        ..default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                children![(
                    Text::new("Reset Simulation"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    TextShadow::default(),
                )]
            )
        ],
    )
}

pub fn reset_sim_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<ResetSimButton>),
    >,
    mut simulator: ResMut<Simulator>,
) {
    for (interaction, mut color, mut border_color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                simulator.reset_system();
                *color = GREEN.into();
                border_color.0 = GREEN.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn pause_resume_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<PauseResumeButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut sim_running: ResMut<SimRunning>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                sim_running.0 = !sim_running.0;
                **text = if sim_running.0 {
                    "Stop Simulation".to_string()
                } else {
                    "Continue".to_string()
                };
                if sim_running.0 {
                    *color = GREEN.into();
                    border_color.0 = GREEN.into();
                } else {
                    *color = RED.into();
                    border_color.0 = RED.into();
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn switch_scene_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<SwitchSceneButton>),
    >,
    mut simulator: ResMut<Simulator>,
) {
    for (interaction, mut color, mut border_color, _) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                simulator.scene_id = (simulator.scene_id + 1) % 2;
                simulator.scene_changed = true;
                *color = GREEN.into();
                border_color.0 = GREEN.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn update_boundary(
    simulator: Res<Simulator>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Mesh3d, With<Boundary>>,
) {
    let half_size = simulator.tank / 2.0;
    let vertices = [
        [-half_size.x, -half_size.y, -half_size.z],
        [half_size.x, -half_size.y, -half_size.z],
        [half_size.x, half_size.y, -half_size.z],
        [-half_size.x, half_size.y, -half_size.z],
        [-half_size.x, -half_size.y, half_size.z],
        [half_size.x, -half_size.y, half_size.z],
        [half_size.x, half_size.y, half_size.z],
        [-half_size.x, half_size.y, half_size.z],
        [half_size.x * simulator.slide_pos, -half_size.y, half_size.z],
        [half_size.x * simulator.slide_pos, -half_size.y, -half_size.z],
        [half_size.x * simulator.slide_pos, half_size.y, -half_size.z],
        [half_size.x * simulator.slide_pos, half_size.y, half_size.z],
    ];

    let positions: Vec<Vec3> = vertices.iter().map(|&v| Vec3::from_array(v)).collect();

    for mesh_handle in &query {
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        }
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
    mut simulator: ResMut<Simulator>,
) {
    commands.spawn(button(&assets));
    simulator.reset_system();

    let mut boundary = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::LineList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let indices: [[u32; 2]; 16] = [
        [0, 1],
        [1, 2],
        [2, 3],
        [3, 0],
        [4, 5],
        [5, 6],
        [6, 7],
        [7, 4],
        [0, 4],
        [1, 5],
        [2, 6],
        [3, 7],
        [8, 9],
        [9, 10],
        [10, 11],
        [11, 8],
    ];

    let flat_indices: Vec<u32> = indices.iter().flat_map(|&[a, b]| [a, b]).collect();
    boundary.insert_indices(bevy::render::mesh::Indices::U32(flat_indices));

    let boundary_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(boundary)),
        MeshMaterial3d(boundary_material),
        Boundary,
    ));

    for pos in simulator.position.iter() {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(simulator.radius).mesh().ico(4).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 30.0 / 255.0, 1.0),
                metallic: 0.2,
                perceptual_roughness: 0.7,
                ..default()
            })),
            Transform::from_xyz(pos.x, pos.y, pos.z),
            Particle,
        ));
    }

    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            range: 20.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0., 3.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
}

pub fn camera_control_system(
    mut query: Query<&mut Transform, With<Camera3d>>,
    input: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    time: Res<Time>,
    mut pitch: Local<f32>,
    mut yaw: Local<f32>,
    mut distance: Local<f32>,
) {
    if *distance == 0.0 {
        *distance = 3.0;
    }
    let mut transform = query.single_mut().unwrap();
    let rotation_speed = 1.0;
    let pitch_speed = 1.0;

    for ev in mouse_wheel_events.read() {
        *distance -= ev.y * 1.0;
        *distance = distance.clamp(2.0, 50.0);
    }

    if input.pressed(KeyCode::ArrowLeft) {
        *yaw += rotation_speed * time.delta_secs();
    }
    if input.pressed(KeyCode::ArrowRight) {
        *yaw -= rotation_speed * time.delta_secs();
    }

    if input.pressed(KeyCode::ArrowUp) {
        *pitch += pitch_speed * time.delta_secs();
    }
    if input.pressed(KeyCode::ArrowDown) {
        *pitch -= pitch_speed * time.delta_secs();
    }

    *pitch = pitch.clamp(
        -std::f32::consts::FRAC_PI_2 + 0.1,
        std::f32::consts::FRAC_PI_2 - 0.1,
    );

    let rotation = Quat::from_rotation_y(*yaw) * Quat::from_rotation_x(*pitch);
    let target = Vec3::new(0.0, 0.0, 0.0);
    let offset = rotation * Vec3::new(0.0, 0.0, *distance);
    transform.translation = target + offset;
    transform.look_at(target, Vec3::Y);
}

pub fn simulation_step(
    mut simulator: ResMut<Simulator>,
    mut query: Query<(&Particle, &mut Transform)>,
    sim_running: Res<SimRunning>,
) {
    if sim_running.0 {
        simulator.simulate_timestep(1.0 / 200.0);
    }

    for (position, (_, mut transform)) in zip(simulator.position.iter(), query.iter_mut()) {
        transform.translation = *position;
    }
}

pub fn scene_refresh_system(
    mut simulator: ResMut<Simulator>,
    commands: Commands,
    query: Query<Entity, With<Particle>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    if simulator.scene_changed {
        simulator.reset_system();
        simulator.scene_changed = false;
        rebuild_particles_system(commands, query, simulator.into(), meshes, materials);
    }
}

pub fn rebuild_particles_system(
    mut commands: Commands,
    query: Query<Entity, With<Particle>>,
    simulator: Res<Simulator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    for pos in simulator.position.iter() {
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(simulator.radius).mesh().ico(4).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.0, 30.0 / 255.0, 1.0),
                metallic: 0.2,
                perceptual_roughness: 0.7,
                ..default()
            })),
            Transform::from_xyz(pos.x, pos.y, pos.z),
            Particle,
        ));
    }
}
