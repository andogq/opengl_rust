use glutin::event::{Event, WindowEvent};
use glutin::platform::run_return::EventLoopExtRunReturn;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::event_loop::{ControlFlow, EventLoop};

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
        F: 'static + FnMut() -> () {
        let event_loop = self.event_loop.unwrap();
        let context = self.context.unwrap();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        println!("WindowEvent::CloseRequested");
                    },
                    _ => ()
                },
                Event::RedrawRequested(_) => {
                    callback();
                    context.swap_buffers().unwrap();
                },
                _ => ()
            };

            match *control_flow {
                ControlFlow::Exit => (),
                _ => {
                    context.window().request_redraw();
                }
            };
        });
    }
}