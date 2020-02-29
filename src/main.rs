extern crate gl;
extern crate libc;
extern crate rand;

mod math;

use math::*;

extern crate glfw;

use rand::Rng;
use gl::types::*;
use glfw::{Action, Context, Key};

mod gl20 {
    include!("../gl_bindings.rs");
}

struct Boid {
    position: Vector2,
    velocity: Vector2,
}

struct Boids {
    glfw: glfw::Glfw,
    window: glfw::Window,
    boids: Vec<Boid>,
}

impl Boids {
    fn new() -> Boids {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, _) = glfw.create_window(1024, 768, "Boids", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

        window.make_current();

        Boids::load_gl(&glfw);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Viewport(0, 0, 1024, 768);
        }

        Boids {
            glfw,
            window,
            boids: Boids::spawn_boids(),
        }
    }

    fn load_gl(glfw: &glfw::Glfw) {
        let loader = |s| glfw.get_proc_address_raw(s);
    
        gl::load_with(loader);
        gl20::load_with(loader);
    }

    fn spawn_boids() -> Vec<Boid> {
        let mut boids: Vec<Boid> = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            boids.push(Boid {
                position: Vector2::new(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0)),
                velocity: Vector2::new(-0.01, -0.01),
            });
        }

        boids
    }

    fn run(&mut self) {
        let mut previous_time: f64;
        let mut dt: f64 = 0.0;
        let mut fps: u32 = 0;
        let mut fps_counter: f64 = 0.0;

        while !self.window.should_close() {
            previous_time = self.glfw.get_time();

            self.update(dt as f32);
            self.render();

            dt = self.glfw.get_time() - previous_time;
            fps_counter += dt;
            fps += 1;

            if fps_counter >= 1.0 {
                println!("FPS: {}", fps);
                fps_counter = 0.0;
                fps = 0;
            }
        }
    }

    fn update(&mut self, dt: f32) {
        self.glfw.poll_events();

        for boid in &mut self.boids {
            (*boid).position += dt * boid.velocity;
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl20::Ortho(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);

            gl20::Begin(gl20::POINTS);
            gl20::Color3f(1.0, 0.0, 0.0);
            for boid in &self.boids {
                gl20::Vertex3f(boid.position.x, boid.position.y, 0.0);
            }
            gl20::End();
        }
        self.window.swap_buffers();
    }
}

fn main() {
    let mut boids = Boids::new();
    boids.run();
}