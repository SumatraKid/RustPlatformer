use macroquad::prelude::*;

use crate::entities::box_collider::BoxCollider;

pub struct Coin {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub value: i16,
    pub collider: BoxCollider,
    pub destroyed: bool,
}