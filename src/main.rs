extern crate gl;
extern crate libc;

mod math;

use math::*;

extern crate glfw;

use gl::types::*;
use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, _) = glfw.create_window(1024, 768, "Boids", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    window.make_current();

    load_gl(&mut glfw);

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Viewport(0, 0, 1024, 768);
    }

    while !window.should_close() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        glfw.poll_events();

        window.swap_buffers();
    }
}

fn load_gl(glfw: &mut glfw::Glfw) {
    let loader = |s| glfw.get_proc_address_raw(s);

    gl::load_with(loader);
    gl::ClearColor::load_with(loader);
    gl::Clear::load_with(loader);
    gl::Viewport::load_with(loader);
}