use rayon::prelude::*;

use crate::aabb::Aabb;
use crate::octree::Octree;
use crate::particle::{Moving, Particle};
use crate::vector::Vector3;
use crate::vector::traits::VectorOps;

pub struct CollisionSolver {
    pub particle_radius: f64,
    pub world_bounds: Aabb,
}

impl CollisionSolver {
    pub fn new(particle_radius: f64, world_bounds: Aabb) -> Self {
        Self {
            particle_radius,
            world_bounds,
        }
    }

    pub fn solve_collisions(&self, particles: &mut [Particle]) {
        let search_radius = self.particle_radius * 2.0;
        let restitution = 0.8;

        let mut tree: Octree<Particle> = Octree::new(self.world_bounds, 200);
        for particle in particles.iter() {
            tree.insert(particle.position, *particle, particle.mass);
        }

        let h = self.world_bounds.half_dimension;
        let min_bound = self.world_bounds.center - Vector3::new(h, h, h);
        let max_bound = self.world_bounds.center + Vector3::new(h, h, h);

        particles.par_iter_mut().for_each(|particle| {
            let query_range = Aabb::new(particle.position, search_radius);

            tree.query_with(&query_range, &mut |_, other_p, _other_mass| {
                if std::ptr::eq(particle, other_p) {
                    return;
                }

                let diff_pos = other_p.position - particle.position;
                let coll_dist = particle.radius + other_p.radius;
                let curr_dist = particle.position.distance_to(&other_p.position);

                if curr_dist >= coll_dist || curr_dist == 0.0 {
                    return;
                }

                let overlap = coll_dist - curr_dist;
                let d_norm = diff_pos / curr_dist;
                let response = overlap * d_norm;

                let v0 = particle.get_velocity();
                let v1 = other_p.get_velocity();

                let v_diff = v1 - v0;
                let diff_dot_v = diff_pos.dot(&v_diff);

                let total_m = particle.mass + other_p.mass;
                let weight1 = other_p.mass / total_m;

                if diff_dot_v >= 0.0 {
                    particle.position -= weight1 * response;
                    return;
                }

                let curr_dist_sq = curr_dist.powi(2);
                let coll_dist_sq = coll_dist.powi(2);
                let dot_v_sq = diff_dot_v.powi(2);
                let v_diff_sq = v_diff.square();

                let mut t: f64 = 0.0;
                if v_diff_sq != 0.0 {
                    t = (diff_dot_v
                        + 0.0f64
                            .max(dot_v_sq - v_diff_sq * (coll_dist_sq - curr_dist_sq))
                            .sqrt())
                        / v_diff_sq;
                }

                particle.position -= v0 * t;

                let normal = particle.position - other_p.position;
                let normal_sq = normal.square();

                let v_rel = v0 - v1;
                let dot = v_rel.dot(&normal);
                let impulse = (2.0 * dot) / (total_m * normal_sq);

                if dot < 0.0 && normal_sq != 0.0 {
                    let v0_new = v0 - impulse * other_p.mass * normal;
                    particle.set_velocity(v0_new);

                    particle.position += v0_new * t;
                    particle.last_position += v0_new * t;
                } else {
                    particle.position += v0 * t;
                }
            });

            let mut v = particle.get_velocity();
            let mut bounced = false;
            let pr = particle.radius;

            if particle.position.x < min_bound.x + pr {
                particle.position.x = min_bound.x + pr;
                v.x = v.x.abs() * restitution;
                bounced = true;
            } else if particle.position.x > max_bound.x - pr {
                particle.position.x = max_bound.x - pr;
                v.x = -v.x.abs() * restitution;
                bounced = true;
            }

            if particle.position.y < min_bound.y + pr {
                particle.position.y = min_bound.y + pr;
                v.y = v.y.abs() * restitution;
                bounced = true;
            } else if particle.position.y > max_bound.y - pr {
                particle.position.y = max_bound.y - pr;
                v.y = -v.y.abs() * restitution;
                bounced = true;
            }

            if particle.position.z < min_bound.z + pr {
                particle.position.z = min_bound.z + pr;
                v.z = v.z.abs() * restitution;
                bounced = true;
            } else if particle.position.z > max_bound.z - pr {
                particle.position.z = max_bound.z - pr;
                v.z = -v.z.abs() * restitution;
                bounced = true;
            }

            if bounced {
                particle.set_velocity(v);
            }
        });
    }
}
