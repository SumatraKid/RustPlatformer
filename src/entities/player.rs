use macroquad::prelude::*;

use crate::entities::box_collider::BoxCollider;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub x_velocity: f32,
    pub y_velocity: f32,
    pub speed: f32,
    pub jump_height: f32,
    pub gravity: f32,
    pub on_ground: bool,
    pub texture: Texture2D,
    pub height: f32,
    pub width: f32,
    pub collider: BoxCollider,
}

impl Player {
    pub fn _movement(&mut self) {
        if is_key_down(KeyCode::Right) {
            self.x_velocity = self.speed;
        }
        else if is_key_down(KeyCode::Left) {
            self.x_velocity = -self.speed;
        }
        else {
            self.x_velocity = 0.0;
        }
        if is_key_down(KeyCode::Down) {
            self.y_velocity = self.speed;
        }
        else if is_key_down(KeyCode::Up) {
            self.y_velocity = -self.speed;
        }
        else {
            self.y_velocity = 0.0;
        }
    }
    pub fn _platformer_movement(&mut self,) {
        // moving right and left
        if is_key_down(KeyCode::Right) {
            self.x_velocity = self.speed;
        }
        else if is_key_down(KeyCode::Left) {
            self.x_velocity = -self.speed;
        }
        else {
            self.x_velocity = 0.0;
        }

        // jumping and gravity
        if is_key_pressed(KeyCode::Up) && self.on_ground == true {
            self.y_velocity = -self.jump_height;
            self.on_ground = false;
        }
        else {
            self.y_velocity += self.gravity;
        }

    }
}