use std::iter::zip;

use crate::simulator::Simulator;
use bevy::input::mouse::MouseWheel;

use bevy::{color::palettes::basic::*, prelude::*, render::render_asset::RenderAssetUsages};

// 仿真运行状态资源
#[derive(Resource, Default)]
pub struct SimRunning(pub bool);

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.8, 0.2, 0.2);

// // 按钮组件
#[derive(Component)]
pub struct SwitchSceneButton;

#[derive(Component)]
pub struct PauseResumeButton;

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
                    // TextColor(Color::srgb(0.1, 0.1, 0.1)),
                    TextColor(Color::WHITE),
                    TextShadow::default(),
                )]
            ),
            (
                Button,
                SwitchSceneButton, // 标记为切换场景按钮
                Node {
                    width: Val::Px(220.0),
                    height: Val::Px(80.0),
                    border: UiRect::all(Val::Px(5.0)),
                    margin: UiRect {
                        left: Val::Px(20.0),
                        top: Val::Px(20.0), // 第二个按钮下移
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
                        top: Val::Px(20.0), // 让按钮排在下方
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
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                simulator.reset_system();
                // reset_wall_box_system();
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
                    // 如果仿真运行，重置边框颜色
                    *color = GREEN.into();
                    border_color.0 = GREEN.into();
                } else {
                    // 如果仿真停止，设置边框颜色为红色
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
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // 切换场景
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
// Add this for Particle
#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct TankBox;

pub fn update_tank_box_system(
    simulator: Res<Simulator>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Mesh3d, With<TankBox>>,
) {
    let half_size = simulator.tank / 2.0;
    let vertices = [
        [-half_size.x, -half_size.y, -half_size.z],
        [half_size.x, -half_size.y, -half_size.z],
        [half_size.x, -half_size.y, half_size.z],
        [-half_size.x, -half_size.y, half_size.z],
        [-half_size.x, half_size.y, -half_size.z],
        [half_size.x, half_size.y, -half_size.z],
        [half_size.x, half_size.y, half_size.z],
        [-half_size.x, half_size.y, half_size.z],
    ];
    let indices: [[u32; 2]; 12] = [
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
    ];
    let positions: Vec<Vec3> = vertices.iter().map(|&v| Vec3::from_array(v)).collect();
    let flat_indices: Vec<u32> = indices.iter().flat_map(|&[a, b]| [a, b]).collect();

    for mesh_handle in &query {
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
            mesh.insert_indices(bevy::render::mesh::Indices::U32(flat_indices.clone()));
        }
    }
}

#[derive(Component)]
pub struct ResetSimButton;

#[derive(Component)]
pub struct WallBox;

pub fn spawn_wall_box(
    commands: &mut Commands,
    simulator: &Simulator,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    // 墙的初始位置和尺寸
    let wall_x = simulator.tank.x * (simulator.slide_pos - 0.5);
    let wall_center = Vec3::new(wall_x, 0.0, 0.0);
    let wall_size = Vec3::new(0.02, simulator.tank.y, simulator.tank.z);

    // 生成线框mesh
    let half_size = wall_size / 2.0;
    let wall_vertices = [
        [-half_size.x, -half_size.y, -half_size.z],
        [half_size.x, -half_size.y, -half_size.z],
        [half_size.x, half_size.y, -half_size.z],
        [-half_size.x, half_size.y, -half_size.z],
        [-half_size.x, -half_size.y, half_size.z],
        [half_size.x, -half_size.y, half_size.z],
        [half_size.x, half_size.y, half_size.z],
        [-half_size.x, half_size.y, half_size.z],
    ];
    let wall_indices: [[u32; 2]; 12] = [
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
    ];
    let positions: Vec<Vec3> = wall_vertices.iter().map(|&v| Vec3::from_array(v)).collect();
    let flat_indices: Vec<u32> = wall_indices.iter().flat_map(|&[a, b]| [a, b]).collect();

    let mut wall_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::LineList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    wall_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    wall_mesh.insert_indices(bevy::render::mesh::Indices::U32(flat_indices));

    let wall_material = materials.add(StandardMaterial {
        base_color: YELLOW.into(),
        unlit: true,
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(wall_mesh)),
        MeshMaterial3d(wall_material),
        WallBox,
        Transform::from_translation(wall_center),
    ));
}

pub fn update_wall_box_system(
    simulator: Res<Simulator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&Mesh3d, &mut Transform), With<WallBox>>,
) {
    if simulator.scene_id != 1 {
        // 不是第二场景时，不更新墙体
        return;
    }

    let wall_x = simulator.tank.x * (simulator.slide_pos - 0.5);
    let wall_center = Vec3::new(wall_x, 0.0, 0.0);
    let wall_size = Vec3::new(0.02, simulator.tank.y, simulator.tank.z);

    let wall_x_render = simulator.tank.x * (simulator.slide_pos - 0.5);
    println!("渲染墙位置: wall_x_render = {}", wall_x_render);

    let half_size = wall_size / 2.0;
    let wall_vertices = [
        [-half_size.x, -half_size.y, -half_size.z],
        [half_size.x, -half_size.y, -half_size.z],
        [half_size.x, half_size.y, -half_size.z],
        [-half_size.x, half_size.y, -half_size.z],
        [-half_size.x, -half_size.y, half_size.z],
        [half_size.x, -half_size.y, half_size.z],
        [half_size.x, half_size.y, half_size.z],
        [-half_size.x, half_size.y, half_size.z],
    ];
    let wall_indices: [[u32; 2]; 12] = [
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
    ];
    let positions: Vec<Vec3> = wall_vertices.iter().map(|&v| Vec3::from_array(v)).collect();
    let flat_indices: Vec<u32> = wall_indices.iter().flat_map(|&[a, b]| [a, b]).collect();

    for (mesh3d, mut transform) in &mut query {
        if let Some(mesh) = meshes.get_mut(&mesh3d.0) {
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
            mesh.insert_indices(bevy::render::mesh::Indices::U32(flat_indices.clone()));
        }
        transform.translation = wall_center;
    }
}

pub fn manage_wall_box_system(
    mut commands: Commands,
    simulator: Res<Simulator>,
    query: Query<Entity, With<WallBox>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if simulator.scene_id == 1 {
        // 如果没有墙体实体则生成
        if query.iter().next().is_none() {
            spawn_wall_box(&mut commands, &simulator, &mut meshes, &mut materials);
        }
    } else {
        // 不是第二场景时，删除所有墙体实体
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}

pub fn reset_wall_box_system(
    mut commands: Commands,
    query: Query<Entity, With<WallBox>>,
    simulator: Res<Simulator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 删除所有旧墙体
    for entity in &query {
        commands.entity(entity).despawn();
    }
    // 只在 scene_id == 1 时重建
    if simulator.scene_id == 1 {
        spawn_wall_box(&mut commands, &simulator, &mut meshes, &mut materials);
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
    mut simulator: ResMut<Simulator>, // 注意：需要把Simulator作为资源传入
) {
    commands.spawn(button(&assets));
    simulator.reset_system();
    // 使用 simulator.tank 作为盒子尺寸
    let box_size = simulator.tank;
    let half_size = box_size / 2.0;

    // 创建盒子边框（使用线段）
    let mut box_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::LineList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let vertices = [
        // 底面4个顶点
        [-half_size.x, -half_size.y, -half_size.z],
        [half_size.x, -half_size.y, -half_size.z],
        [half_size.x, -half_size.y, half_size.z],
        [-half_size.x, -half_size.y, half_size.z],
        // 顶面4个顶点
        [-half_size.x, half_size.y, -half_size.z],
        [half_size.x, half_size.y, -half_size.z],
        [half_size.x, half_size.y, half_size.z],
        [-half_size.x, half_size.y, half_size.z],
    ];
    // 定义12条边（每个立方体有12条边）
    let indices: [[u32; 2]; 12] = [
        // 底面4条边
        [0, 1],
        [1, 2],
        [2, 3],
        [3, 0],
        // 顶面4条边
        [4, 5],
        [5, 6],
        [6, 7],
        [7, 4],
        // 侧面4条边
        [0, 4],
        [1, 5],
        [2, 6],
        [3, 7],
    ];

    // 设置顶点位置
    box_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices
            .iter()
            .map(|&v| Vec3::from_array(v))
            .collect::<Vec<_>>(),
    );

    // 将索引转换为u32格式并设置到网格中
    let flat_indices: Vec<u32> = indices.iter().flat_map(|&[a, b]| [a, b]).collect();
    box_mesh.insert_indices(bevy::render::mesh::Indices::U32(flat_indices));

    // 创建边框材质
    let border_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true, // 不受光照影响
        ..default()
    });

    // 生成边框实体
    commands.spawn((
        Mesh3d(meshes.add(box_mesh)),
        MeshMaterial3d(border_material),
        TankBox, // 添加 TankBox 组件
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
    println!("Spawned {} particles", simulator.num_sphere);

    // 添加灯光
    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            range: 20.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // 添加相机
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
    mut pitch: Local<f32>,    // 新增：记录俯仰角
    mut yaw: Local<f32>,      // 新增：记录水平角
    mut distance: Local<f32>, // 新增：记录摄像头距离
) {
    // 如果 distance 还没初始化，赋初值
    if *distance == 0.0 {
        *distance = 3.0;
    }
    let mut transform = query.single_mut().unwrap(); // 解包Result
    let rotation_speed = 1.0;
    let pitch_speed = 1.0;

    // 鼠标滚轮控制前进/后退
    for ev in mouse_wheel_events.read() {
        // ev.y > 0 向前，ev.y < 0 向后
        *distance -= ev.y * 1.0; // 1.0为缩放速度，可调整
        *distance = distance.clamp(2.0, 50.0); // 限制距离范围
    }

    // 左右旋转（绕Y轴）

    if input.pressed(KeyCode::ArrowLeft) {
        *yaw += rotation_speed * time.delta_secs();
    }
    if input.pressed(KeyCode::ArrowRight) {
        *yaw -= rotation_speed * time.delta_secs();
    }

    // 上下旋转（绕X轴）
    if input.pressed(KeyCode::ArrowUp) {
        *pitch += pitch_speed * time.delta_secs();
    }
    if input.pressed(KeyCode::ArrowDown) {
        *pitch -= pitch_speed * time.delta_secs();
    }
    // 限制俯仰角度，避免翻转
    *pitch = pitch.clamp(
        -std::f32::consts::FRAC_PI_2 + 0.1,
        std::f32::consts::FRAC_PI_2 - 0.1,
    );

    // 计算旋转和位置
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
    // 只有仿真步进受暂停控制
    if sim_running.0 {
        simulator.simulate_timestep(1.0 / 200.0);
    }
    // 无论暂停与否，都同步粒子位置到 Transform
    for (position, (_, mut transform)) in zip(simulator.position.iter(), query.iter_mut()) {
        transform.translation = *position;
    }
}

// 新增：场景刷新系统（不受暂停影响）
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
    }
    // 重建粒子
    rebuild_particles_system(commands, query, simulator.into(), meshes, materials);
}

pub fn rebuild_particles_system(
    mut commands: Commands,
    query: Query<Entity, With<Particle>>,
    simulator: Res<Simulator>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. 删除所有旧粒子实体
    for entity in &query {
        commands.entity(entity).despawn();
    }

    // 2. 重新生成所有粒子实体
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
