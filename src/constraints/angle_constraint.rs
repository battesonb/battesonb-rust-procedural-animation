use super::Constraint;
use crate::joint::Joint;
use lending_iterator::prelude::*;
use macroquad::math::Vec2;

/// Enforces a minimum angle between 3 consecutive points.
#[derive(Clone, Debug)]
pub struct AngleConstraint {
    angle: f32,
    rate: f32,
}

impl AngleConstraint {
    pub fn new(angle: f32, rate: f32) -> Self {
        Self { angle, rate }
    }
}

impl Constraint for AngleConstraint {
    fn apply(&self, joints: &mut Vec<Joint>) {
        let mut iter = joints.windows_mut::<3>();
        while let Some([a, b, c]) = iter.next() {
            let ba = a.pos - b.pos;
            let bc = c.pos - b.pos;

            let angle = ba.angle_between(bc);
            if angle.abs() >= self.angle {
                continue;
            }

            let direction = if ba.perp_dot(bc).is_sign_negative() {
                -1.0
            } else {
                1.0
            };

            let total_angle = ba.to_angle() + self.angle * direction;
            let target = b.pos + bc.length() * Vec2::from_angle(total_angle);

            c.pos = c.pos.lerp(target, self.rate);
        }
    }
}
