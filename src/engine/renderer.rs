use std::ffi::{CStr};
use std::time::{Instant, Duration};

pub struct Renderer {
    initialised: bool,

    fps: u32,
    last_draw: Instant
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            initialised: false,

            fps: 60,
            last_draw: Instant::now()
        }
    }

    pub fn init(&mut self) {
        // Assumes OpenGL bindings have been setup

        // Get OpenGL version
        unsafe {
            let data = gl::GetString(gl::VERSION);
            println!("{}", String::from_utf8(CStr::from_ptr(data as *const i8).to_bytes().to_vec()).unwrap());
        };

        // Set the clear color
        unsafe { gl::ClearColor(0.0, 0.0, 0.0, 1.0) };

        self.initialised = true;
        check_errors();
        println!("Renderer finished initialising");
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.fps = fps;
    }

    pub fn render(&self) {
        // Render code        
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        check_errors();
    }

    pub fn ready_to_render(&self) -> (bool, Instant) {
        // Calculate when next frame should be drawn, and trigger a draw call or wait
        let next_draw = self.last_draw + Duration::from_millis(1000 / self.fps as u64);
        
        return (next_draw <= Instant::now(), next_draw);
    }
}

fn check_errors() {
    loop {
        let error = unsafe { gl::GetError() };

        if error != 0 { println!("[!] OpenGL Error: 0x{:x}", error); }
        else { break; }
    }
}