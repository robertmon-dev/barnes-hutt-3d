use crate::vector::Vector3;
use rand::RngExt;
use std::f32::consts::PI;

pub fn splash_coordinates_with_phi_epsilon(
    i: usize,
    n: usize,
    phi: f32,
    epsilon: f32,
    r: f32,
) -> Vector3 {
    let i_f = i as f32;
    let n_f = n as f32;

    let x = i_f / phi;
    let y = (i_f + epsilon) / (n_f - 1.0 + 2.0 * epsilon);

    let x_i = 2.0 * PI * x;
    let y_i = (1.0 - 2.0 * y).acos();

    let x_c = (r * x_i.cos() * y_i.sin()) as f64;
    let y_c = (r * y_i.sin() * x_i.sin()) as f64;
    let z_c = (r * y_i.cos()) as f64;

    Vector3::new(x_c, y_c, z_c)
}

pub fn splash_coordinates(i: usize, n: usize, r: f32) -> Vector3 {
    let default_phi = (1.0 + 5.0_f32.sqrt()) / 2.0;
    let default_epsilon = 0.0;
    splash_coordinates_with_phi_epsilon(i, n, default_phi, default_epsilon, r)
}

pub fn distribute_across_the_torus(n: usize, r_inn: f32, r_out: f32) -> Vec<Vector3> {
    let mut bodies = Vec::with_capacity(n);
    let mut randomizer = rand::rng();

    for _ in 0..n {
        let theta = randomizer.random_range(0.0..=2.0 * PI);
        let phi = randomizer.random_range(0.0..=2.0 * PI);

        let x = ((r_out + r_inn * phi.cos()) * theta.cos()) as f64;
        let y = ((r_out + r_inn * phi.cos()) * theta.sin()) as f64;
        let z = (r_inn * phi.sin()) as f64;

        bodies.push(Vector3::new(x, y, z));
    }

    bodies
}
