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
        for (index, particle) in particles.iter().enumerate() {
            if !particle.dead {
                tree.insert(particle.position, *particle, particle.mass, index);
            }
        }

        tree.propagate();

        let h = self.world_bounds.half_dimension;
        let min_bound = self.world_bounds.center - Vector3::new(h, h, h);
        let max_bound = self.world_bounds.center + Vector3::new(h, h, h);

        let corrections: Vec<Correction> = particles
            .par_iter()
            .enumerate()
            .flat_map(|(index, particle)| {
                if particle.dead {
                    return Vec::new();
                }

                if !particle.extends(min_bound, max_bound) {
                    return vec![Correction::kill(index)];
                }

                let query_range = Aabb::new(particle.position, search_radius);
                let mut local_corrections = Vec::new();

                tree.query_with(&query_range, &mut |_, other, _| {
                    if let Some(correction) = Self::solve_pair(particle, other, index) {
                        local_corrections.push(correction);
                    }
                });

                let new_acc = tree.calculate_acceleration(
                    particle.position,
                    consts::THETA,
                    consts::EPSILON,
                    consts::G_CONST,
                    consts::MAX_FACTOR,
                );

                local_corrections.push(Correction::from_acceleration(new_acc, index));
                local_corrections
            })
            .collect();

        for corr in corrections {
            if let Some(particle) = particles.get_mut(corr.index()) {
                match corr {
                    Correction::Collision {
                        v0,
                        v0_new,
                        position,
                        last_position,
                        ..
                    } => {
                        if let Some(pos) = position {
                            particle.position += pos;
                        }

                        if v0_new != v0 {
                            particle.last_position = particle.position - v0_new;
                        } else if let Some(last_pos_offset) = last_position {
                            particle.last_position += last_pos_offset;
                        }
                    }
                    Correction::Gravity { acceleration, .. } => {
                        particle.acceleration += acceleration;
                    }
                    Correction::Kill { .. } => {
                        particle.dead = true;
                    }
                }
            }
        }
    }

    fn solve_pair(particle: &Particle, other: &Particle, index: usize) -> Option<Correction> {
        let diff_pos = particle.position - other.position;
        let curr_dist_sq = diff_pos.square();
        let coll_dist = particle.radius + other.radius;

        if curr_dist_sq >= coll_dist.powi(2) || curr_dist_sq == 0.0 {
            return None;
        }

        let curr_dist = curr_dist_sq.sqrt();
        let overlap = coll_dist - curr_dist;
        let normal = diff_pos / curr_dist;

        let v0 = particle.get_velocity();
        let v1 = other.get_velocity();
        let v_rel = v0 - v1;
        let dot = v_rel.dot(&normal);
        let total_m = particle.mass + other.mass;

        let weight = other.mass / total_m;
        let positional_correction = normal * (overlap * weight);

        if dot > 0.0 {
            return Some(Correction::Collision {
                index,
                t: 0.0,
                v0: Vector3::zero(),
                v0_new: Vector3::zero(),
                position: Some(positional_correction),
                last_position: None,
            });
        }

        let impulse_mag = (2.0 * dot) / total_m;
        let v0_new = v0 - normal * (impulse_mag * other.mass);

        Some(Correction::Collision {
            index,
            t: 0.0,
            v0,
            v0_new,
            position: Some(positional_correction),
            last_position: None,
        })
    }
}
