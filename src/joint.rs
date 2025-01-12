use macroquad::prelude::*;

use crate::{
    body::{Body, BodyDescriptor, Side},
    constants::{DEBUG_COLOR, DEBUG_LINE_THICKNESS},
};

#[derive(Clone, Debug)]
pub struct Joint {
    pub pos: Vec2,
    pub radius: f32,
    pub angle: f32,
    pub bodies: Vec<Body>,
}

#[derive(Clone, Debug)]
pub struct JointDescriptor {
    pub radius: f32,
    pub bodies: Vec<BodyDescriptor>,
}

impl Default for JointDescriptor {
    fn default() -> Self {
        Self {
            radius: 5.,
            bodies: Vec::new(),
        }
    }
}

impl JointDescriptor {
    pub fn add_body(&mut self, descriptor: BodyDescriptor) {
        self.bodies.push(descriptor);
    }
}

impl From<JointDescriptor> for Joint {
    fn from(descriptor: JointDescriptor) -> Self {
        Self::new(descriptor)
    }
}

impl Joint {
    pub fn new(descriptor: JointDescriptor) -> Self {
        let JointDescriptor { radius, bodies } = descriptor;
        Self {
            radius,
            pos: Vec2::ZERO,
            angle: 0.,
            bodies: bodies.into_iter().map(Into::into).collect::<Vec<_>>(),
        }
    }

    pub fn draw(&self, side: Side) {
        for body in &self.bodies {
            if body.side == side {
                body.draw();
            }
        }
    }

    pub fn debug_draw(&self) {
        draw_circle_lines(
            self.pos.x,
            self.pos.y,
            self.radius,
            DEBUG_LINE_THICKNESS,
            DEBUG_COLOR,
        );

        for body in &self.bodies {
            body.debug_draw();
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
