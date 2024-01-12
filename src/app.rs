use std::time::{Duration, Instant};

use cgmath::{vec3, ElementWise, MetricSpace};
use minifb::Key::M;
use minifb::Window;
use rand::random;

use crate::camera::Camera;
use crate::renderer::Renderer;
use crate::scene::{Material, Scene, Sphere};
use crate::text::render_into_buffer;
use crate::utils::random_vector3;

pub struct App {
    viewport_width: usize,
    viewport_height: usize,
    camera: Camera,
    renderer: Renderer,
    scene: Scene,
    last_render_time: Duration,
}

const GROUND: Material = Material::lambertian(vec3(0.5, 0.5, 0.5));

const PINK: Material = Material::lambertian(vec3(1.0, 0.0, 1.0));

const BLUE: Material = Material::metal(vec3(0.2, 0.3, 1.0), 0.1);

const ORANGE: Material = Material::metal(vec3(0.8, 0.5, 0.2), 0.1).emissive(20.0);

const BROWN: Material = Material::lambertian(vec3(0.4, 0.2, 0.1));

enum SceneVariant {
    ChernoSun,
    ChernoBalls,
    Rtiaw,
}

const SCENE_VARIANT: SceneVariant = SceneVariant::Rtiaw;

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        let mut scene = Scene::default();

        match SCENE_VARIANT {
            SceneVariant::ChernoSun => {
                scene.spheres.push(Sphere {
                    mat: &PINK,
                    position: vec3(0.0, 0.0, 0.0),
                    radius: 1.0,
                });

                scene.spheres.push(Sphere {
                    mat: &ORANGE,
                    position: vec3(32.0, 32.0, -32.0),
                    radius: 20.0,
                });

                scene.spheres.push(Sphere {
                    mat: &BLUE,
                    position: vec3(0.0, -101.0, 0.0),
                    radius: 100.0,
                });
                scene.global_illumination = false;
            }
            SceneVariant::ChernoBalls => {
                scene.spheres.push(Sphere {
                    mat: &PINK,
                    position: vec3(0.0, 0.0, 0.0),
                    radius: 1.0,
                });

                scene.spheres.push(Sphere {
                    mat: &ORANGE,
                    position: vec3(2.0, 0.0, 0.0),
                    radius: 1.0,
                });

                scene.spheres.push(Sphere {
                    mat: &BLUE,
                    position: vec3(0.0, -101.0, 0.0),
                    radius: 100.0,
                });
                scene.global_illumination = true;
            }
            SceneVariant::Rtiaw => {
                scene.spheres.push(Sphere {
                    position: vec3(0.0, -1000.0, 0.0),
                    radius: 1000.0,
                    mat: &GROUND,
                });

                let scene_center = vec3(4.0, 0.2, 0.0);
                for a in -11..11 {
                    for b in -11..11 {
                        let center = vec3(
                            (a as f32) + 0.9 * random::<f32>(),
                            0.2,
                            (b as f32) + 0.9 * random::<f32>(),
                        );

                        if center.distance(scene_center) > 0.9 {
                            let albedo = random_vector3().mul_element_wise(random_vector3());
                            scene.spheres.push(Sphere {
                                mat: &PINK,
                                position: center,
                                radius: 0.2,
                            })
                        }
                    }
                }

                scene.spheres.push(Sphere {
                    position: vec3(0.0, 1.0, 0.0),
                    radius: 1.0,
                    mat: &BROWN,
                });

                scene.global_illumination = true;
            }
        }

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

        self.camera
            .on_resize(self.viewport_width, self.viewport_height);
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
