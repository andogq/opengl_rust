use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

extern crate gl;

use std::ffi::CStr;

fn main() {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Hello");

    let context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let context = unsafe { context.make_current().unwrap() };

    println!("Pixel format of the window's GL context: {:?}", context.get_pixel_format());

    gl::load_with(|s| context.get_proc_address(s));

    unsafe {
        let data = gl::GetString(gl::VERSION);
        println!("{}", String::from_utf8(CStr::from_ptr(data as *const i8).to_bytes().to_vec()).unwrap());
    };

    unsafe { gl::ClearColor(0.0, 0.0, 0.0, 1.0) };

    el.run(move |event, _, control_flow| {

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => ()
            },
            Event::RedrawRequested(_) => {
                unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
                context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });

}
