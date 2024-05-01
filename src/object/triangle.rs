use glam::Vec3;

use crate::surface::SurfaceMaterial;

use super::Intersectable;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    origin: Vec3,
    edge_a: Vec3,
    edge_b: Vec3,
    normal: Vec3,
    material: SurfaceMaterial,
}

impl Triangle {
    pub fn from_vertices(a: Vec3, b: Vec3, c: Vec3, material: SurfaceMaterial) -> Self {
        let edge_a = b - a;
        let edge_b = c - a;
        let normal = edge_a.cross(edge_b).normalize();

        Triangle {
            origin: a,
            edge_a,
            edge_b,
            normal,
            material,
        }
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, pos: Vec3, ray: Vec3) -> Option<f32> {
        let ray_cross_e2 = ray.cross(self.edge_b);
        let det = self.edge_a.dot(ray_cross_e2);
        if det.abs() < f32::EPSILON {
            return None; // ray is parallel to the triangle
        }

        let inv_det = 1. / det;
        let s = pos - self.origin;
        let u = inv_det * s.dot(ray_cross_e2);
        if u < 0. || u > 1. {
            return None;
        }

        let s_cross_e1 = s.cross(self.edge_a);
        let v = inv_det * ray.dot(s_cross_e1);
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = inv_det * self.edge_b.dot(s_cross_e1);
        if t > f32::EPSILON {
            Some(t)
        } else {
            None
        }
    }

    fn normal(&self, _pos: Vec3) -> Vec3 {
        self.normal
    }

    fn surface(&self) -> &SurfaceMaterial {
        &self.material
    }
}
