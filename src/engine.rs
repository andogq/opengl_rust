mod shader;
mod model;
mod object;
mod camera;
mod renderer;

use shader::Shader;
use model::Model;
use object::Object;
use camera::Camera;
use renderer::Renderer;

use std::collections::HashMap;

use cgmath::Vector3;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder};

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;

pub struct Engine {
    shaders: HashMap<String, Shader>,
    models: Vec<Model>,
    objects: Vec<Object>,
    cameras: Vec<Camera>,
    renderer: Renderer,

    initialised: bool
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            shaders: HashMap::new(),
            models: Vec::new(),
            objects: Vec::new(),
            cameras: Vec::new(),
            renderer: Renderer::new(),

            initialised: false
        }
    }

    pub fn init(&mut self) {
        self.renderer.set_fps(60);

        self.initialised = true;
        println!("Engine finished initialising");
    }

    pub fn run(mut self) {
        //! Window creation need to be done here due to borrow problems with event_loop
        // Create event loop
        let event_loop= EventLoop::new();
        // Create the window and event loop
        let wb = WindowBuilder::new().with_title("Hello").with_inner_size(glutin::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));

        // Load the context
        let context = ContextBuilder::new().with_vsync(true).build_windowed(wb, &event_loop).unwrap();
        let context = unsafe { context.make_current().unwrap() };

        // Load OpenGL function wrapper
        gl::load_with(|s| context.get_proc_address(s));

        // Initialise renderer
        self.renderer.init();

        // Build shaders
        for shader in self.shaders.values_mut() {
            shader.build();
        }

        println!("[+] Beginning main loop");

        // Setup the event loop listener
        event_loop.run(move |event, _, control_flow| {
            // Check what type of event has been called
            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => context.resize(physical_size),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input: glutin::event::KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state,
                            ..
                        },
                        ..
                    } => match (virtual_code, state) {
                        _ => ()
                    },
                    _ => ()
                },
                Event::RedrawRequested(_) => {
                    self.renderer.render();
                    context.swap_buffers().unwrap();
                },
                _ => (),
            }
    
            match *control_flow {
                // Ensure to actually exit
                ControlFlow::Exit => (),
                _ => {
                    match self.renderer.ready_to_render() {
                        (true, _) => context.window().request_redraw(),
                        (false, next_draw) => *control_flow = ControlFlow::WaitUntil(next_draw)
                    }
                }
            }
        });
    }

    pub fn add_shader(&mut self, name: &str) {
        println!("Adding shader `{}`", name);
        self.shaders.entry(String::from(name)).or_insert(Shader::new(name));
    }

    pub fn add_model(&mut self, name: &str, points: &[f32], shader: &str) {

    }

    pub fn add_object(&mut self, model: &str, position: Vector3<f32>, scale: Vector3<f32>) {

    }

    pub fn add_camera(&mut self, name: &str, position: Vector3<f32>) {

    }

    pub fn render(&mut self, camera: &str) {

    }
}