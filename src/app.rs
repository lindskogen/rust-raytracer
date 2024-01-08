use std::time::{Duration, Instant};

use cgmath::vec3;
use minifb::Window;

use crate::camera::Camera;
use crate::renderer::Renderer;
use crate::scene::{Material, Scene, Sphere};
use crate::text::render_into_buffer;

pub struct App {
    viewport_width: usize,
    viewport_height: usize,
    camera: Camera,
    renderer: Renderer,
    scene: Scene,
    last_render_time: Duration,
}

const PINK: Material = Material { roughness: 0.0, albedo: vec3(1.0, 0.0, 1.0), metallic: 0.0 };
const BLUE: Material = Material { roughness: 0.1, albedo: vec3(0.2, 0.3, 1.0), metallic: 0.0 };


impl App {
    pub fn new(width: usize, height: usize) -> Self {
        let mut scene = Scene::default();

        scene.spheres.push(Sphere {
            mat: &PINK,
            position: vec3(0.0, 0.0, 0.0),
            radius: 0.5,
        });

        scene.spheres.push(Sphere {
            mat: &BLUE,
            position: vec3(0.0, -101.0, 0.0),
            radius: 100.0,
        });


        let mut renderer = Renderer::default();
        renderer.on_resize(width, height);

        App {
            renderer,
            viewport_width: width,
            viewport_height: height,
            camera: Camera::new(45.0, 0.1, 100.0),
            scene,
            last_render_time: Duration::ZERO,
        }
    }

    pub fn on_update(&mut self, ts: Duration, window: &mut Window) {
        if self.camera.on_update(ts, window) {
            self.renderer.reset_frame_index();
        }
    }

    pub fn render(&mut self, buffer: &mut Vec<u32>) {
        let time = Instant::now();

        self.camera.on_resize(self.viewport_width, self.viewport_height);
        self.renderer.render(&self.scene, &self.camera, buffer);

        self.last_render_time = time.elapsed();

        self.render_elapsed(buffer);
    }

    fn render_elapsed(&self, buffer: &mut Vec<u32>) {
        let t = self.last_render_time.as_millis() as u8;
        let mut x_offset = 0;
        if t > 9 {
            let c = 0x30 + ((t / 10) % 10);
            render_into_buffer(buffer, c, x_offset, self.viewport_width);
        }
        x_offset = 5;

        let c = 0x30 + (t % 10);
        render_into_buffer(buffer, c, x_offset, self.viewport_width);
    }
}
