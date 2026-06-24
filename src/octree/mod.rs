use crate::aabb::Aabb;
use crate::vector::Vector3;

#[derive(Debug, Clone)]
pub struct Octree<T> {
    boundary: Aabb,
    capacity: usize,
    points: Vec<(Vector3, T, f32)>,
    children: Option<Box<[Octree<T>; 8]>>,

    is_leaf: bool,
    mass: f32,
    center_of_mass: Vector3,
}

impl<T> Octree<T> {
    pub fn new(boundary: Aabb, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            points: Vec::with_capacity(8),
            children: None,
            is_leaf: true,
            mass: 0.0,
            center_of_mass: Vector3::zero(),
        }
    }

    pub fn query_with<F>(&self, range: &Aabb, func: &mut F)
    where
        F: FnMut(&Vector3, &T, &f32),
    {
        if !self.boundary.intersects(range) {
            return;
        }

        for (p, d, mass) in &self.points {
            if range.contains(*p) {
                func(p, d, mass);
            }
        }

        if let Some(ref children) = self.children {
            for child in children.iter() {
                child.query_with(range, func);
            }
        }
    }

    fn subdivide(&mut self) {
        let h = self.boundary.half_dimension / 2.0;
        let c = self.boundary.center;

        let offsets = [
            Vector3::new(-h, -h, -h),
            Vector3::new(h, -h, -h),
            Vector3::new(-h, h, -h),
            Vector3::new(h, h, -h),
            Vector3::new(-h, -h, h),
            Vector3::new(h, -h, h),
            Vector3::new(-h, h, h),
            Vector3::new(h, h, h),
        ];

        let children_array: [Octree<T>; 8] = std::array::from_fn(|i| {
            Octree::new(
                Aabb {
                    center: c + offsets[i],
                    half_dimension: h,
                },
                self.capacity,
            )
        });

        self.children = Some(Box::new(children_array));
    }

    pub fn insert(&mut self, point: Vector3, data: T, mass: f32) -> bool {
        let new_mass = self.mass + mass;
        self.center_of_mass = (self.center_of_mass * self.mass + point * mass) / new_mass;
        self.mass = new_mass;

        if self.points.len() < self.capacity && self.children.is_none() {
            self.points.push((point, data, mass));
            return true;
        }

        if self.children.is_none() {
            self.subdivide();

            let old_points = std::mem::take(&mut self.points);

            for (p, d, mass) in old_points {
                self.insert_to_children(p, d, mass);
            }
        }

        self.insert_to_children(point, data, mass)
    }

    fn insert_to_children(&mut self, point: Vector3, data: T, mass: f32) -> bool {
        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                if child.boundary.contains(point) {
                    return child.insert(point, data, mass);
                }
            }
        }

        false
    }

    pub fn propagate(&mut self) {
        let mut total_mass = 0.0;

        let mut total_x = 0.0;
        let mut total_y = 0.0;
        let mut total_z = 0.0;

        let mut center_of_mass = Vector3::new(total_x, total_y, total_z);

        if self.is_leaf {
            if self.children.is_none() {
                self.mass = total_mass;
                self.center_of_mass = center_of_mass;
            } else {
                for point in self.points.iter() {
                    total_mass += point.2;

                    total_x += point.0.x * point.2;
                    total_y += point.0.y * point.2;
                    total_z += point.0.z * point.2;
                }

                self.mass = total_mass;
                self.center_of_mass = Vector3::new(
                    total_x / total_mass,
                    total_y / total_mass,
                    total_z / total_mass,
                );
            }
        } else {
            if self.children.is_none() {
                return;
            }

            if let Some(ref mut children) = self.children {
                for child in children.iter_mut() {
                    child.propagate();

                    center_of_mass += child.center_of_mass * child.mass;
                    total_mass += child.mass;
                }

                self.mass = total_mass;
                self.center_of_mass = center_of_mass;

                if self.mass > 0.0 {
                    self.center_of_mass -= Vector3::splat(self.mass);
                }
            }
        }
    }
}
