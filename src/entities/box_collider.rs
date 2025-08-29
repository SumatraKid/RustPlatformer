use macroquad::prelude::*;

pub struct BoxCollider {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl BoxCollider {

    pub fn collided(&self, collider: &BoxCollider) -> bool {
        let colliding: bool;
        if self.x + self.width > collider.x &&
            self.x < collider.x + collider.width &&
            self.y + self.height > collider.y &&
            self.y < collider.y + collider.height
            {
            colliding = true;
        }
        else {
            colliding = false;
        }

        return colliding;
    }

}