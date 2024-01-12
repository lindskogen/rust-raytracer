use cgmath::{vec3, Vector3};

pub struct Material {
    pub albedo: Vector3<f32>,
    pub roughness: f32,
    pub metallic: f32,

    pub emission_color: Vector3<f32>,
    pub emission_power: f32,
}

impl Material {
    pub const fn lambertian(albedo: Vector3<f32>) -> Self {
        Self {
            emission_power: 0.0,
            emission_color: vec3(0.0, 0.0, 0.0),
            metallic: 0.0,
            albedo,
            roughness: 0.0,
        }
    }
    pub const fn metal(albedo: Vector3<f32>, roughness: f32) -> Self {
        Self {
            emission_power: 0.0,
            emission_color: vec3(0.0, 0.0, 0.0),
            metallic: 0.0,
            albedo,
            roughness,
        }
    }
    pub const fn emissive(self, emission_power: f32) -> Self {
        Self {
            emission_power,
            emission_color: self.albedo,
            ..self
        }
    }
    pub fn get_emission(&self) -> Vector3<f32> {
        self.emission_color * self.emission_power
    }
}

pub struct Sphere {
    pub position: Vector3<f32>,
    pub radius: f32,
    pub mat: &'static Material,
}

#[derive(Default)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub global_illumination: bool,
}
