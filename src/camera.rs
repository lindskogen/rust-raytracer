use std::time::Duration;

use cgmath::{Deg, Euler, Matrix4, perspective, Point3, Quaternion, Rad, vec2, vec3, vec4, Vector2, Vector3};
use cgmath::prelude::*;
use minifb::{CursorStyle, Key, MouseButton, MouseMode, Window};

pub struct Camera {
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
    inverse_projection: Matrix4<f32>,
    inverse_view: Matrix4<f32>,
    vertical_fov: f32,
    near_clip: f32,
    far_clip: f32,
    position: Vector3<f32>,
    forward_direction: Vector3<f32>,

    // Cached before moving to GPU
    ray_directions: Vec<Vector3<f32>>,
    last_mouse_position: Vector2<f32>,

    pub viewport_width: usize,
    pub viewport_height: usize,
}

impl Camera {
    pub const fn get_position(&self) -> Vector3<f32> { self.position }
    pub const fn get_ray_directions(&self) -> &Vec<Vector3<f32>> { &self.ray_directions }

    pub const fn get_rotation_speed(&self) -> f32 {
        0.3
    }


    pub fn new(vertical_fov: f32, near_clip: f32, far_clip: f32) -> Self {
        Self {
            projection: Matrix4::one(),
            view: Matrix4::one(),
            inverse_projection: Matrix4::one(),
            inverse_view: Matrix4::one(),
            vertical_fov,
            near_clip,
            far_clip,
            position: vec3(0.0, 0.0, 6.0),
            forward_direction: vec3(0.0, 0.0, 1.0),
            ray_directions: Vec::new(),
            last_mouse_position: Vector2::zero(),
            viewport_width: 0,
            viewport_height: 0,
        }
    }

    fn get_mouse_pos(&self, window: &Window) -> Vector2<f32> {
        let (mouse_x, mouse_y) = window.get_mouse_pos(MouseMode::Pass).unwrap();

        vec2(mouse_x, mouse_y)
    }

    pub fn on_resize(&mut self, width: usize, height: usize) {
        if self.viewport_height == height && self.viewport_width == width {
            return;
        }

        self.viewport_width = width;
        self.viewport_height = height;

        self.recalculate_projection();
        self.recalculate_ray_directions();
    }

    pub fn on_update(&mut self, dur: Duration, window: &mut Window) -> bool {
        let ts = dur.as_secs_f32() * 100.0;
        let mouse_pos = self.get_mouse_pos(window);
        let delta = (mouse_pos - self.last_mouse_position) * 0.02;
        self.last_mouse_position = mouse_pos;

        if !window.get_mouse_down(MouseButton::Right) {
            window.set_cursor_style(CursorStyle::Arrow);
            return false;
        }

        window.set_cursor_style(CursorStyle::Crosshair);

        let mut moved = false;

        const UP_DIRECTION: Vector3<f32> = vec3(0.0, 1.0, 0.0);

        let right_direction = self.forward_direction.cross(UP_DIRECTION);
        let speed = 500.0f32;

        if window.is_key_down(Key::W) {
            self.position -= self.forward_direction * speed * ts;
            moved = true;
        }

        if window.is_key_down(Key::S) {
            self.position += self.forward_direction * speed * ts;
            moved = true;
        }

        if window.is_key_down(Key::A) {
            self.position += right_direction * speed * ts;
            moved = true;
        }

        if window.is_key_down(Key::D) {
            self.position -= right_direction * speed * ts;
            moved = true;
        }

        if window.is_key_down(Key::Q) {
            self.position -= UP_DIRECTION * speed * ts;
            moved = true;
        }

        if window.is_key_down(Key::E) {
            self.position += UP_DIRECTION * speed * ts;
            moved = true;
        }


        if delta.x != 0.0 || delta.y != 0.0 {
            let pitch_delta = delta.y * self.get_rotation_speed();
            let yaw_delta = -delta.x * self.get_rotation_speed();

            let pitch_part = Quaternion::from_axis_angle(right_direction, Rad(pitch_delta));
            let yaw_part = Quaternion::from_axis_angle(UP_DIRECTION, Rad(yaw_delta));
            let q = (yaw_part * pitch_part).normalize();

            self.forward_direction = q.rotate_vector(self.forward_direction);

            moved = true;
        }

        if moved {
            self.recalculate_view();
            self.recalculate_ray_directions();
        }

        moved
    }

    fn recalculate_projection(&mut self) {
        self.projection = perspective(Deg(self.vertical_fov),
                                      (self.viewport_width as f32) / (self.viewport_height as f32),
                                      self.near_clip,
                                      self.far_clip);

        self.inverse_projection = self.projection.invert().unwrap();
    }
    fn recalculate_view(&mut self) {
        self.view = Matrix4::look_to_lh(Point3::from_vec(self.position), self.forward_direction, vec3(0.0, 1.0, 0.0));


        self.inverse_view = self.view.invert().unwrap();
    }
    fn recalculate_ray_directions(&mut self) {
        self.ray_directions.resize(self.viewport_width * self.viewport_height, Vector3::zero());

        for y in 0..self.viewport_height {
            for x in 0..self.viewport_width {
                let mut coord = vec2((x as f32) / (self.viewport_width as f32), (y as f32) / (self.viewport_height as f32));

                coord = (coord * 2.0).sub_element_wise(1.0);

                let target = self.inverse_projection * vec4(coord.x, coord.y, 1.0, 1.0);
                let ray_direction = (self.inverse_view * (target.truncate() / target.w).normalize().extend(0.0)).truncate();
                self.ray_directions[x + y * self.viewport_width] = ray_direction;
            }
        }
    }
}
