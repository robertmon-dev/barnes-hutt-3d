mod aabb;
mod collision_solver;
mod logger;
mod octree;
mod particle;
mod renderer;
mod vector;

use particle::Particle;
use rayon::prelude::*;

use vector::Vector3;

use crate::aabb::Aabb;
use crate::collision_solver::CollisionSolver;
use crate::particle::traits::Moving;
use crate::renderer::{Renderer, particle::RenderParticle};
use crate::vector::traits::Distributing;
use crate::vector::traits::VectorOps;

pub struct Simulation {
    buffer: Vec<Particle>,
    world_bounds: Aabb,
    particle_radius: f32,
}

impl Simulation {
    pub fn new(buffer: Vec<Particle>, world_bounds: Aabb, particle_radius: f32) -> Self {
        Self {
            buffer,
            world_bounds,
            particle_radius,
        }
    }

    pub fn step(&mut self, dt: f32, render_buffer: &mut Vec<RenderParticle>) {
        self.buffer.par_iter_mut().for_each(|particle| {
            particle.update(dt);
        });

        let (world_min_bound, world_max_bound) = self.world_bounds.get_bounds();
        self.buffer.par_sort_unstable_by_key(|p| {
            p.position.get_morton_code(world_min_bound, world_max_bound)
        });

        let collision_solver = CollisionSolver::new(self.particle_radius, self.world_bounds);
        collision_solver.solve_collisions(&mut self.buffer);

        render_buffer
            .par_iter_mut()
            .zip(self.buffer.par_iter())
            .for_each(|(render_p, p)| {
                render_p.position = [p.position.x, p.position.y, p.position.z];
            });
    }
}

fn main() {
    logger::Logger::init();

    let world_half_dimension = 1000000.0 * 10.0;
    let particle_count = 100000;
    let particle_radius = 12.0;

    let positions = Vector3::distribute_across_the_sphere(particle_count, world_half_dimension);
    let particles: Vec<Particle> = positions
        .iter()
        .map(|pos| Particle::new(*pos, *pos, Vector3::zero(), 10.0, particle_radius))
        .collect();

    let world_bounds = Aabb::new(Vector3::zero(), world_half_dimension * 3.0);

    let render_buffer = vec![
        RenderParticle {
            position: [0.0, 0.0, 0.0]
        };
        particle_count
    ];

    let simulation = Simulation::new(particles, world_bounds, particle_radius);
    let renderer = Renderer::new(world_half_dimension, particle_count, 10_000.0);

    renderer.run(simulation, render_buffer);
}
