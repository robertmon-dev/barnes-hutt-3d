pub mod particle;

use std::collections::HashMap;

use rayon::prelude::*;
use three_d::*;
use tracing::{debug, info, trace};

use crate::logger::Logger;
use crate::{Simulation, renderer::particle::RenderParticle};

#[derive(Hash, PartialEq, Eq)]
pub enum BodyType {
    Body = 1,
}

pub struct Renderer {
    window: Window,
    camera: Camera,
    instances: HashMap<BodyType, Instances>,
    models: HashMap<BodyType, Gm<InstancedMesh, PhysicalMaterial>>,
    light: DirectionalLight,
    control: OrbitControl,

    frames: usize,
    last_check: std::time::Instant,
    scale: f32,
}

impl Renderer {
    pub fn new(world_half_dimension: f32, bodies_count: usize, scale: f32) -> Self {
        let window = Window::new(WindowSettings {
            title: "Barness-Hutt".to_string(),
            max_size: Some((1280, 720)),
            ..Default::default()
        })
        .unwrap();

        let context = window.gl();
        let camera = Camera::new_perspective(
            window.viewport(),
            vec3(0.0, world_half_dimension, world_half_dimension * 4.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            1.0,
            world_half_dimension * 200.0,
        );

        let cpu_mesh = CpuMesh::sphere(8);
        let material = PhysicalMaterial::new_opaque(
            &context,
            &CpuMaterial {
                albedo: Srgba::new_opaque(50, 150, 255),
                ..Default::default()
            },
        );

        let instances = Instances {
            transformations: vec![Mat4::identity(); bodies_count],
            ..Default::default()
        };

        let mut models = HashMap::new();
        let mut instances_map = HashMap::new();

        let instanced_mesh = InstancedMesh::new(&context, &instances, &cpu_mesh);
        let model = Gm::new(instanced_mesh, material);

        models.insert(BodyType::Body, model);
        instances_map.insert(BodyType::Body, instances);

        let light = DirectionalLight::new(&context, 2.0, Srgba::WHITE, vec3(0.0, -1.0, -1.0));
        let control = OrbitControl::new(camera.target(), 1.0, world_half_dimension * 10.0);

        Self {
            window,
            camera,
            instances: instances_map,
            models,
            light,
            control,
            frames: 0,
            last_check: std::time::Instant::now(),
            scale,
        }
    }

    pub fn run(mut self, mut simulation: Simulation, mut render_buffer: Vec<RenderParticle>) {
        self.window.render_loop(move |mut frame_input| {
            let frame_span = Logger::create_frame_span(self.frames);
            let _entered = frame_span.enter();

            self.camera.set_viewport(frame_input.viewport);
            self.control
                .handle_events(&mut self.camera, &mut frame_input.events);

            debug!("Starting simulation step (dt = 0.016)");
            simulation.step(0.016, &mut render_buffer);

            if let Some(instances) = self.instances.get_mut(&BodyType::Body) {
                trace!(
                    "Updating transformations for: {} particles",
                    render_buffer.len()
                );
                render_buffer
                    .par_iter()
                    .zip(instances.transformations.par_iter_mut())
                    .for_each(|(rp, t)| {
                        if rp.dead {
                            *t = Mat4::from_scale(0.0);
                        } else {
                            *t = Mat4::from_translation(vec3(
                                rp.position[0],
                                rp.position[1],
                                rp.position[2],
                            )) * Mat4::from_scale(self.scale);
                        }
                    });
            }

            let screen = frame_input.screen();
            screen.clear(ClearState::color_and_depth(0.05, 0.05, 0.05, 1.0, 1.0));

            if let (Some(model), Some(instances)) = (
                self.models.get_mut(&BodyType::Body),
                self.instances.get(&BodyType::Body),
            ) {
                model.geometry.set_instances(instances);
                screen.render(&self.camera, &[model], &[&self.light]);
            }

            self.frames += 1;
            if self.last_check.elapsed().as_secs() >= 1 {
                info!(
                    fps = self.frames,
                    total_bodies = render_buffer.len(),
                    "Performance"
                );
                self.frames = 0;
                self.last_check = std::time::Instant::now();
            }

            FrameOutput::default()
        });
    }
}
