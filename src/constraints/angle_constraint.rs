use super::Constraint;
use crate::{body::AttachmentPoint, joint::Joint};
use lending_iterator::prelude::*;
use macroquad::math::Vec2;

/// Enforces a minimum angle between 3 consecutive points.
#[derive(Clone, Debug)]
pub struct AngleConstraint {
    pub(crate) angle: f32,
    /// The rate at which to apply the angle constraint.
    pub(crate) rate: f32,
}

#[derive(Clone, Debug)]
pub struct AngleConstraintDescriptor {
    pub(crate) angle: f32,
    /// The rate at which to apply the angle constraint.
    pub(crate) rate: f32,
}

impl AngleConstraint {
    pub fn new(descriptor: AngleConstraintDescriptor) -> Self {
        let AngleConstraintDescriptor { angle, rate } = descriptor;

        Self { angle, rate }
    }
}

impl Constraint for AngleConstraint {
    fn apply(&mut self, joints: &mut Vec<Joint>, _attachment_point: Option<AttachmentPoint>) {
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
