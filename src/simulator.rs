use bevy::prelude::*;
use rayon::prelude::*;

#[derive(Resource)]
pub struct Simulator {
    pub position: Vec<Vec3>, // Particle Position
    velocity: Vec<Vec3>,     // Particle Velocity
    color: Vec<Vec3>,

    position_: Vec<Vec3>,
    neighbor: Vec<Vec<usize>>,

    cell_x: usize,
    cell_y: usize,
    cell_z: usize,
    num_cell: usize,
    hashtable: Vec<usize>,
    hashtableindex: Vec<usize>,

    pub scene_id: i32,
    pub scene_changed: bool,
    pub tank: Vec3, // tank size
    rel_water: Vec3,
    offset: Vec3,
    pub slide_pos: f32,
    slide_vel: f32,
    slide_dir: i32,

    pub num_sphere: usize,
    pub radius: f32, // radius of particles

    rest_density: f32,
    num: usize,
    ratio: f32, // ratio between max neighbor distance and particle radius
    h: f32,     // max neighbor distance

    solver_iteration: usize,
    relaxation: f32,
    damping: f32,
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
            relaxation: 1e3,
            damping: 0.999,
            gravity: Vec3::new(0.0, -9.81, 0.0), // 默认重力加速度
        }
    }

    fn calc_density(&self, index: usize) -> f32 {
        let mut density = 0.0;
        let pos = self.position_[index];
        for &neighbor_index in &self.neighbor[index] {
            let r = pos - self.position_[neighbor_index];
            let d = poly6(&r, self.h);
            density += d;
        }
        density
    }

    fn calc_constraint(&self, index: usize) -> f32 {
        self.calc_density(index) / self.rest_density - 1.0
    }

    fn calc_grad_constraint(&self, index: usize, neighbor_index: usize) -> Vec3 {
        let grad_c = if neighbor_index == index {
            let mut grad_c = Vec3::ZERO;
            for &neighbor in &self.neighbor[index] {
                let r = self.position_[index] - self.position_[neighbor];
                grad_c += grad_spiky(&r, self.h);
            }
            grad_c
        } else {
            let r = self.position_[index] - self.position_[neighbor_index];
            -grad_spiky(&r, self.h)
        };
        grad_c / self.rest_density
    }

    fn handle_collisions(&mut self) {
        let wall_x_physics = self.tank.x * (self.slide_pos - 0.5);
        println!("物理判定墙位置: wall_x_physics = {}", wall_x_physics);
        for i in 0..self.num_sphere {
            if self.position_[i].x < -0.5 * self.tank.x + self.radius {
                self.position_[i].x = -0.5 * self.tank.x + self.radius;
            }
            if self.position_[i].x > 0.5 * self.tank.x * self.slide_pos - self.radius {
                self.position_[i].x = 0.5 * self.tank.x * self.slide_pos - self.radius;
            }

            if self.position_[i].y < -0.5 * self.tank.y + self.radius {
                self.position_[i].y = -0.5 * self.tank.y + self.radius;
            }
            if self.position_[i].y > 0.5 * self.tank.y - self.radius {
                self.position_[i].y = 0.5 * self.tank.y - self.radius;
            }

            if self.position_[i].z < -0.5 * self.tank.z + self.radius {
                self.position_[i].z = -0.5 * self.tank.z + self.radius;
            }
            if self.position_[i].z > 0.5 * self.tank.z - self.radius {
                self.position_[i].z = 0.5 * self.tank.z - self.radius;
            }
        }
    }

    fn index2grid_offset(&self, index: UVec3) -> usize {
        index.x as usize * self.cell_y * self.cell_z
            + index.y as usize * self.cell_z
            + index.z as usize
    }

    fn build_hashtable(&mut self) {
        self.hashtable.fill(0);
        self.hashtableindex.fill(0);

        for i in 0..self.num_sphere {
            let pos = self.position_[i];
            let index = ((pos + 0.5 * self.tank) / self.h).as_uvec3() + 1;
            let offset = self.index2grid_offset(index);
            self.hashtableindex[offset] += 1;
        }

        let mut prefix_sum = 0;
        for i in 0..self.num_cell {
            prefix_sum += self.hashtableindex[i];
            self.hashtableindex[i] = prefix_sum;
        }
        self.hashtableindex[self.num_cell] = prefix_sum;

        for i in 0..self.num_sphere {
            let pos = self.position_[i];
            let index = ((pos + 0.5 * self.tank) / self.h).as_uvec3() + 1;
            let offset = self.index2grid_offset(index);
            self.hashtableindex[offset] -= 1;
            self.hashtable[self.hashtableindex[offset]] = i;
        }
    }

    fn intergrate_particles(&mut self, dt: f32) {
        for i in 0..self.num_sphere {
            self.velocity[i] += self.gravity * dt;
            self.position_[i] += self.velocity[i] * dt;
        }
    }

    fn detect_neighbor(&mut self) {
        self.handle_collisions();
        self.build_hashtable();
        self.neighbor.clear();
        self.neighbor.resize(self.num_sphere, Vec::new());

        let position_ = &self.position_;
        let hashtable = &self.hashtable;
        let hashtableindex = &self.hashtableindex;
        let h = self.h;
        let tank = self.tank;
        let index2grid_offset = |index: UVec3| {
            index.x as usize * self.cell_y * self.cell_z
                + index.y as usize * self.cell_z
                + index.z as usize
        };

        self.neighbor
            .par_iter_mut()
            .enumerate()
            .for_each(|(p, neighbors)| {
                let pos = position_[p];
                let grid_index = ((pos + 0.5 * tank) / h).as_uvec3() + 1;

                for i in -1..=1 {
                    for j in -1..=1 {
                        for k in -1..=1 {
                            let offset = index2grid_offset(UVec3::new(
                                (grid_index.x as i32 + i) as u32,
                                (grid_index.y as i32 + j) as u32,
                                (grid_index.z as i32 + k) as u32,
                            ));
                            let start = hashtableindex[offset];
                            let end = hashtableindex[offset + 1];

                            for idx in start..end {
                                let neighbor_index = hashtable[idx];
                                if neighbor_index != p {
                                    let d = position_[neighbor_index] - pos;
                                    if d.length_squared() < h * h {
                                        neighbors.push(neighbor_index);
                                    }
                                }
                            }
                        }
                    }
                }
            });

        // for p in 0..self.num_sphere {
        //     let pos = self.position_[p];
        //     let grid_index = ((pos + 0.5 * self.tank) / self.h).as_uvec3() + 1;
        //     for i in -1..=1 {
        //         for j in -1..=1 {
        //             for k in -1..=1 {
        //                 let offset = self.index2grid_offset(UVec3::new(
        //                     (grid_index.x as i32 + i) as u32,
        //                     (grid_index.y as i32 + j) as u32,
        //                     (grid_index.z as i32 + k) as u32,
        //                 ));
        //                 let start = self.hashtableindex[offset];
        //                 let end = self.hashtableindex[offset + 1];

        //                 for idx in start..end {
        //                     let neighbor_index = self.hashtable[idx];
        //                     if neighbor_index != p {
        //                         let d = self.position_[neighbor_index] - pos;
        //                         if d.length_squared() < self.h * self.h {
        //                             self.neighbor[p].push(neighbor_index);
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn constraint_solve(&mut self) {
        let mut lambda = vec![0.0; self.num_sphere];
        let mut delta_pos = vec![Vec3::ZERO; self.num_sphere];

        lambda.par_iter_mut().enumerate().for_each(|(i, lambda_i)| {
            let numerator = self.calc_constraint(i);
            let mut denominator = 0.0;
            for &j in &self.neighbor[i] {
                let grad_c = self.calc_grad_constraint(i, j);
                denominator += grad_c.length_squared();
            }
            denominator += self.calc_grad_constraint(i, i).length_squared();
            denominator += self.relaxation;
            *lambda_i = -numerator / denominator;
        });

        // for i in 0..self.num_sphere {
        //     let numerator = self.calc_constraint(i);
        //     let mut denominator = 0.0;
        //     for &j in &self.neighbor[i] {
        //         let grad_c = self.calc_grad_constraint(i, j);
        //         denominator += grad_c.length_squared();
        //     }
        //     denominator += self.calc_grad_constraint(i, i).length_squared();
        //     denominator += self.relaxation;
        //     lambda[i] = -numerator / denominator;
        // }

        const K: f32 = 1e-5;
        const N: i32 = 4;
        let w = poly6(&vec3(0.3 * self.h, 0.0, 0.0), self.h);

        delta_pos
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, delta_pos_i)| {
                let pos = self.position_[i];
                for &j in &self.neighbor[i] {
                    if j == i {
                        continue;
                    }
                    let r = pos - self.position_[j];
                    let ratio = poly6(&r, self.h) / w;
                    let s_corr = -K * f32::powi(ratio, N);
                    *delta_pos_i += (lambda[i] + lambda[j] + s_corr) * grad_spiky(&r, self.h);
                }
                *delta_pos_i /= self.rest_density;
            });

        // for i in 0..self.num_sphere {
        //     let pos = self.position_[i];
        //     for &j in &self.neighbor[i] {
        //         if j == i {
        //             continue;
        //         }
        //         let r = pos - self.position_[j];
        //         let ratio = poly6(&r, self.h) / w;
        //         let s_corr = -K * f32::powi(ratio, N);
        //         delta_pos[i] += (lambda[i] + lambda[j] + s_corr) * grad_spiky(&r, self.h);
        //     }
        //     delta_pos[i] /= self.rest_density;
        // }

        for i in 0..self.num_sphere {
            self.position_[i] += delta_pos[i];
        }
        self.handle_collisions();
    }

    fn velocity_update(&mut self, dt: f32) {
        for i in 0..self.num_sphere {
            self.velocity[i] = self.damping * (self.position_[i] - self.position[i]) / dt;
            self.position[i] = self.position_[i];
        }
    }

    fn update_particle_colors(&mut self) {
        for i in 0..self.num_sphere {
            let rel_density = f32::clamp(f32::sqrt(self.neighbor[i].len() as f32 / 13.0), 0.7, 1.0);
            self.color[i].x = 1.0 - rel_density;
            self.color[i].y = 1.0 - (1.0 - 30.0 / 255.0) * rel_density;
        }
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
        self.detect_neighbor();
        for _ in 0..self.solver_iteration {
            self.constraint_solve();
        }
        self.velocity_update(dt);
        // self.update_particle_colors();
    }

    fn setup_scene(&mut self) {
        let base =
            -self.tank * 0.5 + self.offset * (vec3(1.0, 1.0, 1.0) - self.rel_water) * self.tank;

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
        const FACTOR: f32 = INV_PI * 315.0 * 5.0 * 5.0 * 5.0 / (64.0 * 9.0 * 9.0 * 9.0);
        let h = self.h;
        self.rest_density = FACTOR * (self.num as f32) / (h * h * h);

        // create particles
        let mut p = 0;
        for i in 0..num_x {
            for j in 0..num_y {
                for k in 0..num_z {
                    self.position[p] = vec3(
                        self.radius + dx * i as f32 + (if j % 2 == 0 { 0.0 } else { self.radius }),
                        self.radius + dy * j as f32,
                        self.radius + dz * k as f32 + (if j % 2 == 0 { 0.0 } else { self.radius }),
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
                self.tank = vec3(0.8, 1.5, 0.8);
                // self.rel_water = vec3(0.5, 0.6, 0.5);
                self.rel_water = vec3(0.2, 0.2, 0.2);
                self.offset = vec3(0.5, 1.0, 0.7);
            } else if self.scene_id == 1 {
                self.tank = vec3(2.0, 1.0, 0.5);
                // self.rel_water = vec3(0.4, 0.6, 1.0);
                self.rel_water = vec3(0.3, 0.3, 0.3);
                self.offset = vec3(0.0, 0.0, 0.5);
            }
            self.slide_pos = 1.0;
            self.slide_vel = 1.0;
            self.slide_dir = -1;
        }
        self.setup_scene();
    }
}
