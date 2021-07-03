use glutin::event::{Event, VirtualKeyCode, WindowEvent};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::event_loop::{ControlFlow, EventLoop};

use std::time;

const FPS: u32 = 60;

pub struct Window {
    event_loop: Option<EventLoop<()>>,
    context: Option<glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>>
}

impl Window {
    pub fn new() -> Window {
        let event_loop = EventLoop::new();
        // Create the window and event loop
        let wb = WindowBuilder::new().with_title("Hello").with_inner_size(glutin::dpi::LogicalSize::new(640, 480));

        // Load the context
        let context = ContextBuilder::new().with_vsync(true).build_windowed(wb, &event_loop).unwrap();
        let context = unsafe { context.make_current().unwrap() };

        // Load OpenGL function wrapper
        gl::load_with(|s| context.get_proc_address(s));

        Window {
            event_loop: Some(event_loop),
            context: Some(context)
        }
    }

    pub fn run<F>(self, mut callback: F) where 
        F: 'static + FnMut(&Vec<VirtualKeyCode>) -> () {
        let event_loop = self.event_loop.unwrap();
        let context = self.context.unwrap();

        let mut last_draw: time::Instant = time::Instant::now();

        let mut pressed = Vec::new();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input: glutin::event::KeyboardInput{virtual_keycode: Some(key), state, ..}, .. } => {
                        let index = pressed.iter().position(|&k| k == key);

                        match state {
                            glutin::event::ElementState::Pressed => if index == None { pressed.push(key); },
                            glutin::event::ElementState::Released => if let Some(i) = index { pressed.remove(i); }
                        }
                    }
                    _ => ()
                },
                Event::RedrawRequested(_) => {
                    callback(&pressed);
                    context.swap_buffers().unwrap();

                    last_draw = time::Instant::now();
                },
                _ => ()
            };

            match *control_flow {
                ControlFlow::Exit => (),
                _ => {
                    let next_draw = last_draw + time::Duration::from_millis(1000 / FPS as u64);
        
                    if next_draw <= time::Instant::now() { context.window().request_redraw(); }
                    else { *control_flow = ControlFlow::WaitUntil(next_draw); }
                }
            };
        });
    }
}