pub mod sphere;

use enum_dispatch::enum_dispatch;
use glam::Vec3;

use crate::surface::SurfaceMaterial;

#[derive(Debug, Clone, PartialEq)]
#[enum_dispatch]
pub enum Object {
    Sphere(sphere::Sphere),
}

#[enum_dispatch(Object)]
pub trait Intersectable {
    fn intersect(&self, pos: Vec3, dir: Vec3) -> Option<f32>;
    fn normal(&self, pos: Vec3) -> Vec3;
    fn surface(&self) -> &SurfaceMaterial;
}
