use cgmath::{ElementWise, InnerSpace, vec3, Vector3, Vector4, Zero};
use cgmath::prelude::*;
use rand::random;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::ray::Ray;
use crate::scene::Scene;

struct HitPayload {
    hit_distance: f32,
    world_position: Vector3<f32>,
    world_normal: Vector3<f32>,
    object_index: usize,
}

pub struct Renderer {
    frame_index: usize,
    accumulation_data: Vec<Vector4<f32>>,
}

impl Default for Renderer {
    fn default() -> Self
    {
        Self {
            frame_index: 1,
            accumulation_data: Vec::new(),
        }
    }
}

impl Renderer {
    pub fn reset_frame_index(&mut self) {
        self.frame_index = 1;
    }

    pub fn on_resize(&mut self, width: usize, height: usize) {
        self.accumulation_data.resize(width * height, Vector4::zero());
    }

    fn render_pixels_in_parallel(&self, scene: &Scene, camera: &Camera) -> Vec<(usize, usize, Vector4<f32>)> {
        (0..camera.viewport_height).into_par_iter().flat_map_iter(|y| {
            (0..camera.viewport_width).map(move |x| {
                (x, y, self.per_pixel(scene, camera, x, y))
            })
        }).collect()
    }

    pub fn render(&mut self, scene: &Scene, camera: &Camera, buffer: &mut Vec<u32>) {
        if self.frame_index == 1 {
            self.accumulation_data.fill(Vector4::zero());
        }


        let pixels = self.render_pixels_in_parallel(scene, camera);

        for (x, y, color) in pixels {
            self.accumulation_data[x + y * camera.viewport_width] += color;
            let mut acc_color = self.accumulation_data[x + y * camera.viewport_width];

            acc_color /= self.frame_index as f32;

            write_to_buffer_inverted(camera.viewport_width, camera.viewport_height, buffer, x, y, acc_color);
        }

        self.frame_index += 1;
    }


    fn per_pixel(&self, scene: &Scene, camera: &Camera, x: usize, y: usize) -> Vector4<f32> {
        let mut ray = Ray { origin: camera.get_position(), direction: camera.get_ray_directions()[x + y * camera.viewport_width] };


        let mut light = Vector3::zero();
        let mut contribution = vec3::<f32>(1.0, 1.0, 1.0);

        let bounces = 5;

        for _ in 0..bounces {
            match self.trace_ray(&ray, scene) {
                Some(payload) if payload.hit_distance > 0.0 => {
                    let sphere = &scene.spheres[payload.object_index];
                    let material = sphere.mat;


                    contribution.mul_assign_element_wise(material.albedo);
                    light += material.get_emission();

                    ray.origin = payload.world_position + payload.world_normal * 0.0001;

                    // ray.direction = reflect(ray.direction, payload.world_normal + material.roughness * random_vector3(-0.5, 0.5)).normalize()
                    ray.direction = (payload.world_normal + random_in_unit_sphere()).normalize()
                }
                _ => {

                    // light += vec3(0.6, 0.7, 0.9).mul_element_wise(contribution);
                    break;
                }
            }
        }

        light.extend(1.0)
    }

    fn trace_ray(&self, ray: &Ray, scene: &Scene) -> Option<HitPayload> {
        let mut closest = None;

        let mut hit_distance = f32::MAX;

        for sphere_index in 0..scene.spheres.len() {
            let sphere = &scene.spheres[sphere_index];

            let origin = ray.origin - sphere.position;

            let a = ray.direction.dot(ray.direction);
            let b = 2.0 * origin.dot(ray.direction);
            let c = origin.dot(origin) - sphere.radius * sphere.radius;

            let discriminant = b * b - 4.0 * a * c;

            if discriminant < 0.0 {
                continue;
            }

            let closest_t = (-b - discriminant.sqrt()) / (2.0 * a);

            if closest_t > 0.0 && closest_t < hit_distance {
                hit_distance = closest_t;
                closest = Some(sphere_index)
            }
        }


        closest.map(|hit| self.closest_hit(ray, scene, hit_distance, hit))
    }

    fn closest_hit(&self, ray: &Ray, scene: &Scene, hit_distance: f32, object_index: usize) -> HitPayload {
        let closest_sphere = &scene.spheres[object_index];

        let origin = ray.origin - closest_sphere.position;
        let world_position = origin + ray.direction * hit_distance;

        let world_normal = world_position.normalize() + closest_sphere.position;

        HitPayload {
            hit_distance,
            object_index,
            world_position,
            world_normal,
        }
    }
}

fn write_to_buffer_inverted(width: usize, height: usize, buffer: &mut Vec<u32>, x: usize, y: usize, mut acc_color: Vector4<f32>) {
    acc_color.x = acc_color.x.clamp(0.0, 1.0);
    acc_color.y = acc_color.y.clamp(0.0, 1.0);
    acc_color.z = acc_color.z.clamp(0.0, 1.0);
    acc_color.w = acc_color.w.clamp(0.0, 1.0);

    buffer[x + (height - y - 1) * width] = convert_to_rgba(acc_color);
}


fn convert_to_rgba(color: Vector4<f32>) -> u32 {
    let r = (color.x * 255.0) as u8;
    let g = (color.y * 255.0) as u8;
    let b = (color.z * 255.0) as u8;
    let a = (color.w * 255.0) as u8;

    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn reflect(incident: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    incident - 2.0 * normal.dot(incident) * normal
}


fn random_vector3(min: f32, max: f32) -> Vector3<f32> {
    vec3(random::<f32>() * (max - min) + min, random::<f32>() * (max - min) + min, random::<f32>() * (max - min) + min)
}

fn random_in_unit_sphere() -> Vector3<f32> {
    random_vector3(-1.0, 1.0).normalize()
}
