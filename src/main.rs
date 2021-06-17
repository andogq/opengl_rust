use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, PossiblyCurrent};

use std::ffi::CStr;

fn main() {
    // Create the window and event loop
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("Hello");

    // Load the context
    let context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
    let context = unsafe { context.make_current().unwrap() };

    // Load OpenGL function wrapper
    gl::load_with(|s| context.get_proc_address(s));

    // Get OpenGL version
    unsafe {
        let data = gl::GetString(gl::VERSION);
        println!("{}", String::from_utf8(CStr::from_ptr(data as *const i8).to_bytes().to_vec()).unwrap());
    };

    // Set the clear color
    unsafe { gl::ClearColor(0.0, 0.0, 0.0, 1.0) };

    // Run the event loop
    el.run(move |event, _, control_flow| {
        // Check what type of event has been called
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => ()
            },
            Event::RedrawRequested(_) => {
                draw(&context);
            },
            _ => (),
        }
    });
}

fn draw(context: &glutin::ContextWrapper<PossiblyCurrent, glutin::window::Window>) {
    unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
    context.swap_buffers().unwrap();
}
