use macroquad::prelude::*;

use crate::body::Body;

#[derive(Clone, Debug)]
pub struct Joint {
    pub pos: Vec2,
    pub radius: f32,
    pub angle: f32,
    pub bodies: Vec<Body>,
}

impl Joint {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        Self::new2(Vec2::new(x, y), radius)
    }

    pub fn new2(pos: Vec2, radius: f32) -> Self {
        Self {
            pos,
            radius,
            angle: 0.0,
            bodies: Vec::new(),
        }
    }

    pub fn draw(&self) {
        for body in &self.bodies {
            body.draw();
        }
    }

    pub fn with_body(mut self, body: Body) -> Self {
        self.add_body(body);
        self
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
}
