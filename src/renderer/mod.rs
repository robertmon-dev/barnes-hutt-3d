pub mod particle;

use std::collections::HashMap;

use rayon::prelude::*;
use three_d::*;

use crate::{Simulation, renderer::particle::RenderParticle};

#[derive(Hash, PartialEq, Eq)]
pub enum BodyType {
    Body = 1,
    Heavy = 2,
    Idle = 3,
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
}

impl Renderer {
    pub fn new(world_half_dimension: f32, bodies_count: usize) -> Self {
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
            world_half_dimension * 20.0,
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
        }
    }

    pub fn run(mut self, mut simulation: Simulation, mut render_buffer: Vec<RenderParticle>) {
        let visual_scale = 10_000.0;

        self.window.render_loop(move |mut frame_input| {
            self.camera.set_viewport(frame_input.viewport);
            self.control
                .handle_events(&mut self.camera, &mut frame_input.events);

            simulation.step(0.016, &mut render_buffer);

            if let Some(instances) = self.instances.get_mut(&BodyType::Body) {
                render_buffer
                    .par_iter()
                    .zip(instances.transformations.par_iter_mut())
                    .for_each(|(rp, t)| {
                        *t = Mat4::from_translation(vec3(
                            rp.position[0],
                            rp.position[1],
                            rp.position[2],
                        )) * Mat4::from_scale(visual_scale);
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
                println!("FPS: {}", self.frames);
                self.frames = 0;
                self.last_check = std::time::Instant::now();
            }

            FrameOutput::default()
        });
    }
}
