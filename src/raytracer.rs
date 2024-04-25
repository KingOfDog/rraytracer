use crate::object_store::OBJECT_STORE;

pub struct Raytracer {}

impl Raytracer {
    pub fn new(width: u32, height: u32) -> Self {
        unsafe { OBJECT_STORE.read_scene(width, height) };

        unsafe { OBJECT_STORE.camera.calculate_first_ray() };

        Raytracer {}
    }

    pub fn render(&self) -> Vec<u8> {
        unsafe { OBJECT_STORE.camera.trace() };

        let frame_buffer = unsafe { OBJECT_STORE.camera.frame_buffer() };
        frame_buffer
            .iter()
            .flat_map(|pixel| pixel.to_rgb_bytes())
            .collect()
    }

    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            OBJECT_STORE.camera.resize(width, height);
        }
    }
}
