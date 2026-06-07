use crate::aabb::Aabb;
use crate::vector::Vector3;

#[derive(Debug, Clone)]
pub struct Octree<T> {
    boundary: Aabb,
    capacity: usize,
    points: Vec<(Vector3, T, f32)>,
    children: Option<Box<[Octree<T>; 8]>>,

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

    pub fn propagate(&mut self) {}
}
