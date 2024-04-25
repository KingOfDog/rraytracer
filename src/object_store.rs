use glam::{vec3, Vec3};
use once_cell::sync::Lazy;

use crate::{
    camera::{Camera, CameraDescriptor},
    color::Color,
    light::Light,
    object::{sphere::Sphere, Object},
    surface::SurfaceMaterial,
};

pub static mut OBJECT_STORE: Lazy<ObjectStore> = Lazy::new(|| ObjectStore::new());

pub struct ObjectStore {
    pub camera: Camera,
    objects: Vec<Object>,
    lights: Vec<Light>,
}

impl ObjectStore {
    pub fn new() -> Self {
        ObjectStore {
            camera: Camera::default(),
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn read_scene(&mut self, width: u32, height: u32) {
        println!("width: {}, height: {}", width, height);
        let camera = Camera::new(&CameraDescriptor {
            width,
            height,
            vfov: 50.,
            eye_pointer: vec3(0., 0., 10.),
            look_pointer: vec3(0., 0., 0.),
            up_pointer: vec3(0., 1., 0.),
            max_level: 5,
            background: Color::new(150., 10., 50.),
            near: 0.0001,
            far: 10000.,
        });
        self.camera = camera;

        let light = Light::new(vec3(10., 5., 5.), 1.);
        self.lights.push(light);

        self.objects.push(
            Sphere::new(
                Vec3::ZERO,
                3.,
                SurfaceMaterial {
                    ambient: Color::new(0., 0., 0.),
                    diffuse: Color::new(200., 0., 0.),
                    specular: Color::new(200., 0., 0.),
                    specular_power: 15.,
                    reflection: 0.6,
                    transparency: 0.,
                },
            )
            .into(),
        );

        self.objects.push(
            Sphere::new(
                vec3(2., 1., 5.),
                1.,
                SurfaceMaterial {
                    ambient: Color::new(0., 50., 0.),
                    diffuse: Color::new(0., 100., 0.),
                    specular: Color::new(30., 40., 30.),
                    specular_power: 2.,
                    reflection: 1.,
                    transparency: 0.,
                },
            )
            .into(),
        );

        self.objects.push(
            Sphere::new(
                vec3(-2., 1., 5.),
                1.,
                SurfaceMaterial {
                    ambient: Color::new(0., 0., 50.),
                    diffuse: Color::new(0., 0., 100.),
                    specular: Color::new(30., 30., 40.),
                    specular_power: 2.,
                    reflection: 0.3,
                    transparency: 0.,
                },
            )
            .into(),
        );
    }

    pub fn lights_mut(&mut self) -> impl Iterator<Item = &mut Light> {
        self.lights.iter_mut()
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
}
