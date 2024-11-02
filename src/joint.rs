use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Joint {
    pub pos: Vec2,
    pub radius: f32,
    pub angle: f32,
    // TODO: Each joint should be able to have "sub-bodies" with their own constraints/rules.
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
        }
    }
}
