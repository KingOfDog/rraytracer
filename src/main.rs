mod camera;
mod color;
mod consts;
mod light;
mod object;
mod object_store;
mod raytracer;
mod surface;

use std::rc::Rc;

use eyre::Result;

use raytracer::Raytracer;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window = Rc::new(WindowBuilder::new().build(&event_loop)?);

    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let raytracer = Raytracer::new(window.inner_size().width, window.inner_size().height);

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                surface
                    .resize(width.try_into().unwrap(), height.try_into().unwrap())
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();

                let frame = raytracer
                    .render()
                    .chunks(3)
                    .map(|rgb| (rgb[0] as u32) << 16 | (rgb[1] as u32) << 8 | rgb[2] as u32)
                    .collect::<Vec<_>>();
                buffer.copy_from_slice(frame.as_slice());

                buffer.present().unwrap();
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } if window_id == window.id() => elwt.exit(),
            _ => {}
        }
    })?;

    Ok(())
}
