use crate::color::Color;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SurfaceMaterial {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub specular_power: f32,
    pub reflection: f32,
    pub transparency: f32,
}
