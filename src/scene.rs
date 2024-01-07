use cgmath::Vector3;

pub struct Material {
    pub albedo: Vector3<f32>,
    pub roughness: f32,
    pub metallic: f32,
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
