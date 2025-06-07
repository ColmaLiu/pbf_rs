use bevy::prelude::*;

pub struct Simulator {
    pub position: Vec<Vec3>, // Particle Position
    velocity: Vec<Vec3>, // Particle Velocity
    color: Vec<Vec3>,

    position_: Vec<Vec3>,
    neighbor: Vec<Vec<usize>>,

    cell_x: usize,
    cell_y: usize,
    cell_z: usize,
    num_cell: usize,
    hashtable: Vec<i32>,
    hashtableindex: Vec<i32>,

    pub scene_id: i32,
    pub scene_changed: bool,
    pub tank: Vec3, // tank size
    rel_water: Vec3,
    offset: Vec3,
    slide_pos: f32,
    slide_vel: f32,
    slide_dir: i32,

    num_sphere: usize,
    pub radius: f32, // radius of particles

    rest_density: f32,
    num: usize,
    ratio: f32, // ratio between max neighbor distance and particle radius
    h: f32, // max neighbor distance

    solver_iteration: usize,
    relaxation: f32,
    damping: f32,
    viscosity_coeff: f32,
    gravity: Vec3,
}

const INV_PI: f32 = 0.318301;

fn poly6(r: &Vec3, h: f32) -> f32 {
    let r2 = r.length_squared();
    let h2 = h * h;
    let diff = h2 - r2;
    if diff < 0.0 {
        return 0.0;
    }
    let coeff = 315.0 * INV_PI / 64.0;
    let diff3 = diff * diff * diff;
    let h4 = h2 * h2;
    let h9 = h4 * h4 * h;
    coeff * diff3 / h9
}

fn grad_spiky(r: &Vec3, h: f32) -> Vec3 {
    let r2 = r.length_squared();
    let h2 = h * h;
    if r2 > h2 {
        return Vec3::ZERO;
    }
    let coeff = -45.0 * INV_PI;
    let r_norm = r2.sqrt();
    let diff = h - r_norm;
    let h3 = h * h * h;
    let h6 = h3 * h3;
    coeff * diff * diff / (h6 * r_norm.max(1e-24)) * r
}

impl Simulator {
    pub fn new() -> Self {
        // a new simulator without scene initialization
        // the scene will be initialized in reset_system()
        Self {
            position: Vec::new(),
            velocity: Vec::new(),
            color: Vec::new(),

            position_: Vec::new(),
            neighbor: Vec::new(),

            cell_x: 0,
            cell_y: 0,
            cell_z: 0,
            num_cell: 0,
            hashtable: Vec::new(),
            hashtableindex: Vec::new(),

            scene_id: 0,
            scene_changed: true,
            tank: Vec3::ZERO,
            rel_water: Vec3::ZERO,
            offset: Vec3::ZERO,
            slide_pos: 0.0,
            slide_vel: 0.0,
            slide_dir: 0,

            num_sphere: 0,
            radius: 0.015,

            rest_density: 0.0,
            num: 9,
            ratio: 3.0,
            h: 0.0,

            solver_iteration: 5,
            relaxation: 2e5,
            damping: 0.999,
            viscosity_coeff: 1e-8,
            gravity: Vec3::new(0.0, -9.81, 0.0), // 默认重力加速度
        }
    }

    fn calc_density(&self, index: usize) -> f32 {
        todo!()
    }

    fn calc_constraint(&self, index: usize) -> f32 {
        todo!()
    }

    fn calc_grad_constraint(&self, index: usize, neighbor_index: usize) -> Vec3 {
        todo!()
    }

    fn handle_collisions(&self) {
        todo!()
    }

    fn index2grid_offset(&self, index: UVec3) -> usize {
        todo!()
    }
    fn build_hashtable(&self) {
        todo!()
    }

    fn intergrate_particles(&mut self, dt: f32) {
        for i in 0..self.num_sphere {
            self.velocity[i] += self.gravity * dt;
            self.position_[i] += self.velocity[i] * dt;
        }
    }

    fn detect_neighbor(&self) {
        todo!()
    }

    fn constraint_solve(&self) {
        todo!()
    }

    fn velocity_update(&mut self, dt: f32) {
        for i in 0..self.num_sphere {
            self.velocity[i] = self.damping * (self.position_[i] - self.position[i]) / dt;
            self.position[i] = self.position_[i];
        }
    }

    fn update_particle_colors(&self) {
        todo!()
    }

    pub fn simulate_timestep(&mut self, dt: f32) {
        if self.scene_id == 1 {
            self.slide_pos += self.slide_dir as f32 * self.slide_vel * dt;
            if self.slide_pos > 1.0 {
                self.slide_dir = -1;
                self.slide_pos = 2.0 - self.slide_pos;
            } else if self.slide_pos < 0.5 {
                self.slide_dir = 1;
                self.slide_pos = 1.0 - self.slide_pos;
            }
        }
        self.intergrate_particles(dt);
        // self.detect_neighbor();
        // for iter in 0..self.solver_iteration {
        //     self.constraint_solve();
        // }
        self.velocity_update(dt);
        // self.update_particle_colors();
    }

    fn setup_scene(&mut self) {
        let base = -self.tank * 0.5 + self.offset * (vec3(1.0, 1.0, 1.0) - self.rel_water) * self.tank;

        let dx = 2.0 * self.radius;
        let dy = 3.0_f32.sqrt() / 2.0 * dx;
        let dz = dx;

        let num_x = (self.rel_water.x * self.tank.x / dx).floor() as usize;
        let num_y = (self.rel_water.y * self.tank.y / dy).floor() as usize;
        let num_z = (self.rel_water.z * self.tank.z / dz).floor() as usize;

        // update object member attributes
        self.num_sphere = num_x * num_y * num_z;
        self.h = self.radius * self.ratio;
        self.cell_x = (self.tank.x / self.h).ceil() as usize + 2;
        self.cell_y = (self.tank.y / self.h).ceil() as usize + 2;
        self.cell_z = (self.tank.z / self.h).ceil() as usize + 2;
        self.num_cell = self.cell_x * self.cell_y * self.cell_z;

        // update particle array
        self.position.clear();
        self.position.resize(self.num_sphere, Vec3::ZERO);
        self.velocity.clear();
        self.velocity.resize(self.num_sphere, Vec3::ZERO);
        self.color.clear();
        self.color.resize(self.num_sphere, Vec3::ZERO);

        self.position_.clear();
        self.position_.resize(self.num_sphere, Vec3::ZERO);
        self.neighbor.clear();
        self.neighbor.resize(self.num_sphere, Vec::new());

        self.hashtable.clear();
        self.hashtable.resize(self.num_sphere, 0);
        self.hashtableindex.clear();
        self.hashtableindex.resize(self.num_cell + 1, 0);

        // the rest density can be assigned after scene initialization
        let factor = INV_PI * 315.0 * 5.0 * 5.0 * 5.0 / (64.0 * 9.0 * 9.0 * 9.0);
        let h = self.h;
        self.rest_density = factor * (self.num as f32) / (h * h * h);

        // create particles
        let mut p = 0;
        for i in 0..num_x {
            for j in 0..num_y {
                for k in 0..num_z {
                    self.position[p] = vec3(
                        self.radius + dx * i as f32 + (if j % 2 == 0 {
                            0.0
                        } else {
                            self.radius
                        }),
                        self.radius + dy * j as f32,
                        self.radius + dz * k as f32 + (if j % 2 == 0 {
                            0.0
                        } else {
                            self.radius
                        })
                    ) + base;
                    p += 1;
                }
            }
        }

        self.position_ = self.position.clone();
    }

    pub fn reset_system(&mut self) {
        if self.scene_changed {
            if self.scene_id == 0 {
                self.tank = vec3(1.0, 2.0, 1.0);
                self.rel_water = vec3(0.4, 0.8, 0.5);
                self.offset = vec3(0.5, 1.0, 0.7);
            } else if self.scene_id == 1 {
                self.tank = vec3(2.0, 1.0, 0.5);
                self.rel_water = vec3(0.4, 0.6, 1.0);
                self.offset = vec3(0.0, 0.0, 0.5);
            }
            self.slide_pos = 1.0;
            self.slide_vel = 1.0;
            self.slide_dir = -1;
        }
        self.setup_scene();
    }
}
