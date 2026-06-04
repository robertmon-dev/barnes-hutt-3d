mod aabb;
mod collision_solver;
mod distribution;
mod octree;
mod particle;
mod renderer;
mod vector;

use particle::Particle;
use rayon::prelude::*;
use three_d::*;

use vector::Vector3;

use crate::aabb::Aabb;
use crate::collision_solver::CollisionSolver;
use crate::distribution::splash_coordinates;
use crate::particle::traits::Moving;
use crate::renderer::particle::RenderParticle;
use crate::vector::traits::VectorOps;

pub struct Simulation {
    buffer: Vec<Particle>,
    world_bounds: Aabb,
    particle_radius: f64,
}

impl Simulation {
    pub fn new(buffer: Vec<Particle>, world_bounds: Aabb, particle_radius: f64) -> Self {
        Self {
            buffer,
            world_bounds,
            particle_radius,
        }
    }

    pub fn step(&mut self, dt: f64, render_buffer: &mut Vec<RenderParticle>) {
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
                render_p.position = [
                    p.position.x as f32,
                    p.position.y as f32,
                    p.position.z as f32,
                ];
            });
    }
}

fn main() {
    let world_half_dimension = 1000000.0 * 10.0;
    let particle_count = 100000;
    let particle_radius = 12.0;

    let positions: Vec<Vector3> = (0..particle_count)
        .map(|i| splash_coordinates(i, particle_count, world_half_dimension))
        .collect();

    let particles: Vec<Particle> = positions
        .iter()
        .map(|pos| Particle::new(*pos, *pos, Vector3::zero(), 10.0, particle_radius))
        .collect();

    let world_bounds = Aabb::new(Vector3::zero(), world_half_dimension as f64 * 3.0);

    let mut render_buffer = vec![
        RenderParticle {
            position: [0.0, 0.0, 0.0]
        };
        particle_count
    ];
    let mut simulation = Simulation::new(particles, world_bounds, particle_radius);

    let window = Window::new(WindowSettings {
        title: "Barness-Hutt".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, world_half_dimension, world_half_dimension * 4.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        1.0,
        world_half_dimension * 20.0,
    );
    let mut control = OrbitControl::new(camera.target(), 1.0, world_half_dimension * 10.0);

    let light = DirectionalLight::new(&context, 2.0, Srgba::WHITE, vec3(0.0, -1.0, -1.0));

    let cpu_mesh = CpuMesh::sphere(8);
    let material = PhysicalMaterial::new_opaque(
        &context,
        &CpuMaterial {
            albedo: Srgba::new_opaque(50, 150, 255),
            ..Default::default()
        },
    );

    let mut instances = Instances {
        transformations: vec![Mat4::identity(); particle_count],
        ..Default::default()
    };
    let instanced_mesh = InstancedMesh::new(&context, &instances, &cpu_mesh);
    let mut model = Gm::new(instanced_mesh, material);

    let mut frames = 0;
    let mut last_check = std::time::Instant::now();

    window.render_loop(move |mut frame_input| {
        camera.set_viewport(frame_input.viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        simulation.step(0.016, &mut render_buffer);
        let visual_scale = 10_000.0;

        render_buffer
            .par_iter()
            .zip(instances.transformations.par_iter_mut())
            .for_each(|(rp, t)| {
                *t = Mat4::from_translation(vec3(rp.position[0], rp.position[1], rp.position[2]))
                    * Mat4::from_scale(visual_scale);
            });

        model.geometry.set_instances(&instances);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.05, 0.05, 0.05, 1.0, 1.0))
            .render(&camera, &[&model], &[&light]);

        frames += 1;
        if last_check.elapsed().as_secs() >= 1 {
            println!("FPS: {}", frames);
            frames = 0;
            last_check = std::time::Instant::now();
        }

        FrameOutput::default()
    });
}
