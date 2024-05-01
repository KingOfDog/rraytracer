use glam::{vec3, Vec3};
use once_cell::sync::Lazy;

use crate::{
    camera::{Camera, CameraDescriptor},
    color::Color,
    light::Light,
    object::{sphere::Sphere, triangle::Triangle, Object},
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
            eye_pointer: vec3(0., 1., 2.),
            look_pointer: vec3(0., 0.5, 0.),
            up_pointer: vec3(0., 1., 0.),
            max_level: 5,
            background: Color::new(150., 10., 50.),
            near: 0.0001,
            far: 10000.,
        });
        self.camera = camera;

        let light = Light::new(vec3(0., 1.2, 0.), 0.12);
        self.lights.push(light);

        let cornell_box = tobj::load_obj("CornellBox-Original.obj", &tobj::GPU_LOAD_OPTIONS)
            .expect("object file to be loaded");
        let (models, materials) = cornell_box;
        let materials = materials.expect("materials to be loaded");

        let materials = materials
            .iter()
            .map(|m| {
                let ambient = m.ambient.unwrap_or_default();
                let diffuse = m.diffuse.unwrap_or_default();
                println!("material: {:?}", m.name);
                println!("other: {:?}", m.unknown_param);
                SurfaceMaterial {
                    ambient: ambient.into(),
                    diffuse: diffuse.into(),
                    specular: m.specular.unwrap_or_default().into(),
                    specular_power: m.shininess.unwrap_or(0.),
                    reflection: 0.,
                    transparency: m.dissolve.unwrap_or(0.),
                }
            })
            .collect::<Vec<_>>();

        for model in models {
            println!("model: {:?}", model.name);
            let mesh = model.mesh;

            let positions = mesh
                .positions
                .chunks(3)
                .map(|p| vec3(p[0], p[1], p[2]))
                .collect::<Vec<_>>();

            let normals = mesh
                .normals
                .chunks(3)
                .map(|n| vec3(n[0], n[1], n[2]))
                .collect::<Vec<_>>();

            let indices = mesh
                .indices
                .chunks(3)
                .map(|i| (i[0] as usize, i[1] as usize, i[2] as usize))
                .collect::<Vec<_>>();

            let material = &materials[mesh.material_id.unwrap_or(0)];

            let triangles = indices
                .iter()
                .map(|(i0, i1, i2)| {
                    let a = positions[*i0];
                    let b = positions[*i1];
                    let c = positions[*i2];

                    Triangle::from_vertices(a, b, c, material.clone()).into()
                })
                .collect::<Vec<_>>();

            self.objects.extend(triangles);
        }

        // self.objects.push(
        //     Sphere::new(
        //         Vec3::ZERO,
        //         3.,
        //         SurfaceMaterial {
        //             ambient: Color::new(0., 0., 0.),
        //             diffuse: Color::new(200., 0., 0.),
        //             specular: Color::new(200., 0., 0.),
        //             specular_power: 15.,
        //             reflection: 0.6,
        //             transparency: 0.,
        //         },
        //     )
        //     .into(),
        // );

        // self.objects.push(
        //     Sphere::new(
        //         vec3(2., 1., 5.),
        //         1.,
        //         SurfaceMaterial {
        //             ambient: Color::new(0., 50., 0.),
        //             diffuse: Color::new(0., 100., 0.),
        //             specular: Color::new(30., 40., 30.),
        //             specular_power: 2.,
        //             reflection: 1.,
        //             transparency: 0.,
        //         },
        //     )
        //     .into(),
        // );

        // self.objects.push(
        //     Sphere::new(
        //         vec3(-2., 1., 5.),
        //         1.,
        //         SurfaceMaterial {
        //             ambient: Color::new(0., 0., 50.),
        //             diffuse: Color::new(0., 0., 100.),
        //             specular: Color::new(30., 30., 40.),
        //             specular_power: 2.,
        //             reflection: 0.3,
        //             transparency: 0.,
        //         },
        //     )
        //     .into(),
        // );

        // self.objects.push(
        //     Triangle::from_vertices(
        //         vec3(2., 1., 6.),
        //         vec3(-2., 1., 6.),
        //         vec3(0., 0., 6.),
        //         SurfaceMaterial {
        //             ambient: Color::new(0., 50., 0.),
        //             diffuse: Color::new(0., 200., 0.),
        //             specular: Color::new(30., 60., 0.),
        //             specular_power: 6.,
        //             reflection: 0.2,
        //             transparency: 0.,
        //         },
        //     )
        //     .into(),
        // );
    }

    pub fn lights_mut(&mut self) -> impl Iterator<Item = &mut Light> {
        self.lights.iter_mut()
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
}
