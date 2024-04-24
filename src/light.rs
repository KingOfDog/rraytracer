use glam::Vec3;

use crate::{
    object::{Intersectable, Object},
    object_store::OBJECT_STORE,
};

pub struct Light {
    pub position: Vec3,
    pub brightness: f32,
    distance_to_light: f32,
}

impl Light {
    pub fn new(position: Vec3, brightness: f32) -> Self {
        Light {
            position,
            brightness,
            distance_to_light: 0.,
        }
    }

    pub fn lightray(&mut self, pos: Vec3) -> Vec3 {
        self.distance_to_light = (self.position - pos).length();
        (self.position - pos).normalize()
    }

    pub fn brightness(&self, source: &Object, pos: Vec3, ray: Vec3) -> f32 {
        for object in unsafe { OBJECT_STORE.objects() } {
            if object != source {
                let Some(s) = object.intersect(pos, ray) else {
                    continue;
                };
                if s <= self.distance_to_light {
                    return 0.;
                }
            }
        }

        self.brightness
    }
}
