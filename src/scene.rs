use cgmath::Vector3;

pub struct Material {
    pub albedo: Vector3<f32>,
    pub roughness: f32,
    pub metallic: f32,

    pub emission_color: Vector3<f32>,
    pub emission_power: f32
}

impl Material {
    pub fn get_emission(&self) -> Vector3<f32> {
        self.emission_color * self.emission_power
    }
}


pub struct Sphere {
    pub position: Vector3<f32>,
    pub radius: f32,
    pub mat: &'static Material
}


#[derive(Default)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
}
