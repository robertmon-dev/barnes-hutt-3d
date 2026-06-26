use rayon::prelude::*;

pub mod consts;
pub mod correction;

use crate::aabb::Aabb;
use crate::attractor::correction::Correction;
use crate::octree::Octree;
use crate::particle::{Particle, traits::Moving};
use crate::vector::Vector3;
use crate::vector::traits::VectorOps;

pub struct Attractor {
    pub particle_radius: f32,
    pub world_bounds: Aabb,
}

impl Attractor {
    pub fn new(particle_radius: f32, world_bounds: Aabb) -> Self {
        Self {
            particle_radius,
            world_bounds,
        }
    }

    pub fn update(&self, particles: &mut Vec<Particle>) {
        let search_radius = self.particle_radius * 2.0;

        let mut tree: Octree<Particle> = Octree::new(self.world_bounds, 200, 0, consts::TREE_DEPTH);
        for particle in particles.iter() {
            if !particle.dead {
                tree.insert(particle.position, *particle, particle.mass);
            }
        }

        tree.propagate();

        let h = self.world_bounds.half_dimension;
        let min_bound = self.world_bounds.center - Vector3::new(h, h, h);
        let max_bound = self.world_bounds.center + Vector3::new(h, h, h);

        let corrections: Vec<Correction> = particles
            .par_iter_mut()
            .enumerate()
            .map(|(index, particle)| {
                if particle.dead {
                    return Vec::new();
                }

                if !particle.extends(min_bound, max_bound) {
                    particle.dead = true;
                    return Vec::new();
                }

                let query_range = Aabb::new(particle.position, search_radius);

                let mut local_corrections = Vec::new();

                tree.query_with(&query_range, &mut |_, other, _| {
                    let correction = Self::solve_pair(particle, other, index);
                    local_corrections.push(correction);
                });

                let new_acc = tree.calculate_acceleration(
                    particle.position,
                    consts::THETA,
                    consts::EPSILON,
                    consts::G_CONST,
                    consts::MAX_FACTOR,
                );
                particle.add_acceleration(new_acc);

                local_corrections
            })
            .flatten()
            .collect();

        for corr in corrections {
            if let Some(particle) = particles.get_mut(corr.index) {
                particle.apply_correction(corr);
            }
        }
    }

    fn solve_pair(particle: &mut Particle, other: &Particle, index: usize) -> Correction {
        if std::ptr::eq(particle, other) {
            return Correction::new(0.0, Vector3::zero(), Vector3::zero(), None, None, index);
        }

        let diff_pos = other.position - particle.position;
        let coll_dist = particle.radius + other.radius;
        let curr_dist = particle.position.distance_to(&other.position);

        if curr_dist >= coll_dist || curr_dist == 0.0 {
            return Correction::new(0.0, Vector3::zero(), Vector3::zero(), None, None, index);
        }

        let overlap = coll_dist - curr_dist;
        let d_norm = diff_pos / curr_dist;
        let response = overlap * d_norm;

        let v0 = particle.get_velocity();
        let v1 = other.get_velocity();

        let v_diff = v1 - v0;
        let diff_dot_v = diff_pos.dot(&v_diff);

        let total_m = particle.mass + other.mass;
        let weight1 = other.mass / total_m;

        if diff_dot_v >= 0.0 {
            return Correction::new(
                0.0,
                Vector3::zero(),
                Vector3::zero(),
                Some(weight1 * response),
                None,
                index,
            );
        }

        let curr_dist_sq = curr_dist.powi(2);
        let coll_dist_sq = coll_dist.powi(2);
        let dot_v_sq = diff_dot_v.powi(2);
        let v_diff_sq = v_diff.square();

        let mut t: f32 = 0.0;
        if v_diff_sq != 0.0 {
            t = (diff_dot_v
                + 0.0f32
                    .max(dot_v_sq - v_diff_sq * (coll_dist_sq - curr_dist_sq))
                    .sqrt())
                / v_diff_sq;
        }

        let normal = particle.position - other.position;
        let normal_sq = normal.square();

        let v_rel = v0 - v1;
        let dot = v_rel.dot(&normal);
        let impulse = (2.0 * dot) / (total_m * normal_sq);
        let v0_new = v0 - impulse * other.mass * normal;

        if dot < 0.0 && normal_sq != 0.0 {
            Correction::new(
                t,
                v0,
                v0_new,
                Some((v0_new * t) - (v0 * t)),
                Some(v0_new * t),
                index,
            )
        } else {
            Correction::new(t, v0, v0_new, Some((v0_new * t) - (v0 * t)), None, index)
        }
    }
}
