use std::iter::zip;

use crate::simulator::Simulator;
use bevy::input::mouse::MouseWheel;
use bevy::{prelude::*, render::render_asset::RenderAssetUsages};

// Add this for Particle
#[derive(Component)]
pub struct Particle;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut simulator: ResMut<Simulator>, // 注意：需要把Simulator作为资源传入
) {
    // 盒子尺寸
    let box_size = 1.0;
    let half_size = box_size / 2.0;

    // 创建盒子边框（使用线段）
    let mut box_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::LineList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    // 定义盒子的8个顶点
    let vertices = [
        // 底面4个顶点
        [-half_size, -half_size, -half_size],
        [half_size, -half_size, -half_size],
        [half_size, -half_size, half_size],
        [-half_size, -half_size, half_size],
        // 顶面4个顶点
        [-half_size, half_size, -half_size],
        [half_size, half_size, -half_size],
        [half_size, half_size, half_size],
        [-half_size, half_size, half_size],
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
    ));

    simulator.reset_system();

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
    mut query: Query<(&Particle, &mut Transform)>
) {
    simulator.simulate_timestep(1.0 / 200.0);
    for (position, (_, mut transform)) in zip(simulator.position.iter(), query.iter_mut()) {
        transform.translation = *position;
    }
}
