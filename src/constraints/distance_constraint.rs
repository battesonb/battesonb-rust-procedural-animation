use crate::{body::AttachmentPoint, joint::Joint};

use super::Constraint;
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Forward,
    Backwards,
}

#[derive(Clone, Debug, Default)]
pub struct DistanceConstraint {
    /// The distance between each joint.
    pub(crate) distance: f32,
    /// The direction in which to apply the distance constraint.
    pub(crate) direction: Direction,
    /// The rate at which to apply the distance constraint.
    pub(crate) rate: f32,
}

#[derive(Clone, Debug)]
pub struct DistanceConstraintDescriptor {
    /// The distance between each joint.
    pub(crate) distance: f32,
    /// The direction in which to apply the distance constraint.
    pub(crate) direction: Direction,
    /// The rate at which to apply the distance constraint.
    pub(crate) rate: f32,
}

impl Default for DistanceConstraintDescriptor {
    fn default() -> Self {
        Self {
            distance: 100.0,
            rate: 1.0,
            direction: Direction::Forward,
        }
    }
}

impl DistanceConstraint {
    pub fn new(descriptor: DistanceConstraintDescriptor) -> Self {
        let DistanceConstraintDescriptor {
            distance,
            direction,
            rate,
        } = descriptor;

        Self {
            distance,
            direction,
            rate,
        }
    }

    #[inline]
    pub fn distance_squared(&self) -> f32 {
        self.distance * self.distance
    }

    fn apply_to_pair(&self, joint: &mut Joint, source: Vec2) {
        let delta = joint.pos - source;
        let distance_squared = delta.length_squared();
        if distance_squared <= self.distance_squared() {
            return;
        }

        let distance = distance_squared.sqrt();
        let target = source + self.distance * delta / distance;

        joint.pos = joint.pos.lerp(target, self.rate);
    }
}

impl Constraint for DistanceConstraint {
    fn apply(&mut self, joints: &mut Vec<Joint>, _attachment_point: Option<AttachmentPoint>) {
        match self.direction {
            Direction::Backwards => {
                for i in (0..(joints.len() - 1)).rev() {
                    let source = joints[i + 1].pos;
                    self.apply_to_pair(&mut joints[i], source);
                }
            }
            Direction::Forward => {
                for i in 0..(joints.len() - 1) {
                    let source = joints[i].pos;
                    self.apply_to_pair(&mut joints[i + 1], source);
                }
            }
        }
    }
}
