use glam::Vec3;

use crate::{
    color::Color,
    consts::FAR_AWAY,
    object::{Intersectable, Object},
    object_store::OBJECT_STORE,
};

#[derive(Default)]
pub struct Camera {
    pub eye_pointer: Vec3,
    pub look_pointer: Vec3,
    pub up_pointer: Vec3,

    pub near: f32,
    pub far: f32,

    pub hfov: f32,
    pub vfov: f32,

    pub width: u32,
    pub height: u32,

    first_ray: Vec3,
    gaze: Vec3,

    scrnx: Vec3,
    scrny: Vec3,

    pub max_level: u32,

    pub background: Color,

    frame_buffer: Vec<Color>,
}

impl Camera {
    pub fn new(desc: &CameraDescriptor) -> Self {
        let aspect = desc.width as f32 / desc.height as f32;
        let hfov = (2. * (aspect * (desc.vfov / 2.).to_radians().tan()).atan()).to_degrees();
        Camera {
            eye_pointer: desc.eye_pointer,
            look_pointer: desc.look_pointer,
            up_pointer: desc.up_pointer,
            near: desc.near,
            far: desc.far,
            vfov: desc.vfov,
            hfov,
            width: desc.width,
            height: desc.height,
            max_level: desc.max_level,
            background: desc.background,
            ..Default::default()
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        let aspect = self.width as f32 / self.height as f32;
        let hfov = (2. * (aspect * (self.vfov / 2.).to_radians().tan()).atan()).to_degrees();
        self.hfov = hfov;
    }

    pub fn calculate_first_ray(&mut self) -> Vec3 {
        let gaze = self.look_pointer - self.eye_pointer;
        self.gaze = gaze.normalize();

        self.scrnx = self.gaze.cross(self.up_pointer).normalize();
        self.scrny = self.scrnx.cross(self.gaze).normalize();

        let dist = gaze.length() * 2.;
        let magnitude = dist * self.hfov.to_radians().tan() / self.width as f32;
        self.scrnx = self.scrnx * magnitude;

        let magnitude = dist * self.vfov.to_radians().tan() / self.height as f32;
        self.scrny = self.scrny * magnitude;

        let offset = -self.scrnx * self.width as f32 / 2. + self.scrny * self.height as f32 / 2.;
        self.first_ray = self.look_pointer - self.eye_pointer + offset;

        self.first_ray
    }

    pub fn trace(&mut self) {
        self.frame_buffer = vec![Color::default(); self.width as usize * self.height as usize];
        //self.depth_buffer = vec![f32::MAX; self.width as usize * self.height as usize];

        println!("width = {}, height = {}", self.width, self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let ray =
                    (self.first_ray + x as f32 * self.scrnx - y as f32 * self.scrny).normalize();

                let color = self.intersect_and_shade(None, self.eye_pointer, ray, 0);

                self.frame_buffer[(y * self.width + x) as usize] = color;
            }
        }
    }

    fn intersect_and_shade(
        &self,
        source: Option<&Object>,
        pos: Vec3,
        ray: Vec3,
        level: u32,
    ) -> Color {
        let intersection = self.intersect(source, pos, ray);

        let color = match intersection {
            Some((distance, hit, ray, normal, object_hit)) if distance > 0. => {
                self.shade(hit, ray, normal, object_hit, level)
            }
            _ => self.background,
        };

        color
    }

    fn intersect(
        &self,
        source: Option<&Object>,
        pos: Vec3,
        ray: Vec3,
    ) -> Option<(f32, Vec3, Vec3, Vec3, &Object)> {
        let mut ss = FAR_AWAY;

        let mut object_hit = None;

        for object in unsafe { OBJECT_STORE.objects() } {
            if Some(object) != source {
                let Some(s) = object.intersect(pos, ray) else {
                    continue;
                };

                if s > 0. && s <= ss {
                    ss = s;
                    object_hit = Some(object);
                }
            }
        }

        let object_hit = object_hit?;

        let hit = pos + ray * ss;

        let normal = object_hit.normal(hit);

        Some((ss, hit, ray, normal, object_hit))
    }

    fn shade(&self, pos: Vec3, ray: Vec3, normal: Vec3, object: &Object, level: u32) -> Color {
        let k = -2. * ray.dot(normal);
        let reflected_ray = normal * k + ray;

        let surface = object.surface();

        let mut color = surface.ambient;

        for light in unsafe { OBJECT_STORE.lights_mut() } {
            let light_ray = light.lightray(pos);

            let mut diffuse = normal.dot(light_ray);

            if diffuse > 0. {
                let brightness = light.brightness(object, pos, light_ray);
                diffuse *= brightness;
                color += surface.diffuse * diffuse;

                let specular = reflected_ray.dot(light_ray);
                if specular > 0. {
                    let specular = specular.powf(surface.specular_power);
                    color += surface.specular * specular;
                }
            }
        }

        let k = surface.reflection;
        if k > 0. && level < self.max_level {
            let reflection_color =
                self.intersect_and_shade(Some(object), pos, reflected_ray, level + 1);
            color += reflection_color * k;
        }

        let k = surface.transparency;
        if k > 0. {
            color *= 1. - k;
            let trans_color = self.intersect_and_shade(Some(object), pos, ray, level);
            color += trans_color * k;
        }

        color
    }

    pub fn frame_buffer(&self) -> &[Color] {
        &self.frame_buffer
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CameraDescriptor {
    pub eye_pointer: Vec3,
    pub look_pointer: Vec3,
    pub up_pointer: Vec3,

    pub near: f32,
    pub far: f32,

    pub vfov: f32,

    pub width: u32,
    pub height: u32,

    pub max_level: u32,

    pub background: Color,
}
