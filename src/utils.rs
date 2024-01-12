use cgmath::{InnerSpace, vec3, Vector3};
use rand::random;

pub fn pcg_hash(input: u32) -> u32 {
    let state = input.wrapping_mul(747796495).wrapping_add(2891336453);
    let word = ((state >> ((state >> 28) + 4)) ^ state).wrapping_mul(277803737);

    (word >> 22) ^ word
}

pub fn pcg_float(input: &mut u32) -> f32 {
    *input = pcg_hash(*input);

    return (*input as f32) / (u32::MAX as f32);
}

pub fn pcg_vec3(input: &mut u32) -> Vector3<f32> {
    vec3(pcg_float(input) * 2.0 - 1.0, pcg_float(input) * 2.0 - 1.0, pcg_float(input) * 2.0 - 1.0)
}

pub fn reflect(incident: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    incident - 2.0 * normal.dot(incident) * normal
}


pub fn random_vector3_in_range(min: f32, max: f32) -> Vector3<f32> {
    vec3(random::<f32>() * (max - min) + min, random::<f32>() * (max - min) + min, random::<f32>() * (max - min) + min)
}

pub fn random_vector3() -> Vector3<f32> {
    vec3(random(), random(), random())
}


pub fn random_in_unit_sphere() -> Vector3<f32> {
    random_vector3_in_range(-1.0, 1.0).normalize()
}
