use glam::Vec3;

use crate::surface::SurfaceMaterial;

use super::Intersectable;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: SurfaceMaterial,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: SurfaceMaterial) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, pos: Vec3, dir: Vec3) -> Option<f32> {
        let adj = pos - self.center;

        let b = adj.x * dir.x + adj.y * dir.y + adj.z * dir.z;
        let t = b.powi(2) - adj.length_squared() + self.radius.powi(2);
        if t < 0. {
            return None;
        }

        let s1 = -b - t.sqrt();
        if s1 > 0. {
            return Some(s1);
        }

        let s2 = -b + t.sqrt();
        if s2 > 0. {
            return Some(s2);
        }

        None
    }

    fn normal(&self, pos: Vec3) -> Vec3 {
        (pos - self.center) / self.radius
    }

    fn surface(&self) -> &SurfaceMaterial {
        &self.material
    }
}
