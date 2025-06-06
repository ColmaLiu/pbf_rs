use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
    },
};
use rand::Rng;

#[derive(Component)]
struct FluidParticle;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 盒子尺寸
    let box_size = 5.0;
    let half_size = box_size / 2.0;
    
    // 创建盒子边框（使用线段）
    let mut box_mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::LineList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD);
    
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
        [0, 1], [1, 2], [2, 3], [3, 0],
        // 顶面4条边
        [4, 5], [5, 6], [6, 7], [7, 4],
        // 侧面4条边
        [0, 4], [1, 5], [2, 6], [3, 7]
    ];
    
    // 设置顶点位置
    box_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices.iter().map(|&v| Vec3::from_array(v)).collect::<Vec<_>>(),
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

    // 创建流体粒子（球体）
    let particle_count = 100;
    let particle_radius = 0.1;

    let mut rng = rand::rng();
    
    for _ in 0..particle_count {
        // 在盒子内部随机位置生成粒子
        let x = rng.random_range(-box_size/2.0..box_size/2.0);
        let y = rng.random_range(-box_size/2.0..box_size/2.0);
        let z = rng.random_range(-box_size/2.0..box_size/2.0);
        
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(particle_radius).mesh().ico(4).unwrap())),
            MeshMaterial3d(
                materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 30.0 / 255.0, 1.0),
                    metallic: 0.2,
                    perceptual_roughness: 0.7,
                    ..default()
                }),
            ),
            Transform::from_xyz(x, y, z),
            FluidParticle,
        ));
    }

    // 添加灯光
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // 添加相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
}
