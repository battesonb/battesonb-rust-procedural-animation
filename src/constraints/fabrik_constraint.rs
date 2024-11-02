use crate::{body::AttachmentPoint, joint::Joint};

use super::{Constraint, Direction, DistanceConstraint};
use macroquad::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct FabrikConstraint {
    /// The inner distance constraint.
    pub(crate) distance_constraint: DistanceConstraint,
    /// The angle of the target relative to the body.
    pub(crate) target_angle: f32,
    /// The desired distance between the first and last point.
    pub(crate) target_distance: f32,
    /// Max distance from the current target and the desired target.
    pub(crate) max_distance: f32,
    /// The current target position.
    pub(crate) current_target_position: Vec2,
}

#[derive(Clone, Debug)]
pub struct FabrikConstraintDescriptor {
    /// The distance between each joint/joint.
    pub joint_distance: f32,
    /// The rate at which to apply the distance constraint in each direction.
    pub rate: f32,
    /// The angle of the target relative to the body.
    pub target_angle: f32,
    /// The desired distance between the first and last point.
    pub target_distance: f32,
    /// Max distance from the current target and the desired target.
    pub max_distance: f32,
}

impl Default for FabrikConstraintDescriptor {
    fn default() -> Self {
        Self {
            joint_distance: 10.,
            rate: 0.25,
            target_angle: 0.,
            target_distance: 20.,
            max_distance: 20.,
        }
    }
}

impl FabrikConstraint {
    pub fn new(descriptor: FabrikConstraintDescriptor) -> Self {
        let FabrikConstraintDescriptor {
            joint_distance,
            rate,
            target_angle,
            target_distance,
            max_distance,
        } = descriptor;

        Self {
            target_angle,
            target_distance,
            max_distance,
            current_target_position: Vec2::ZERO,
            distance_constraint: DistanceConstraint {
                distance: joint_distance,
                rate,
                ..Default::default()
            },
        }
    }
}

impl Constraint for FabrikConstraint {
    fn apply(&mut self, joints: &mut Vec<Joint>, attachment_point: Option<AttachmentPoint>) {
        let attachment_point = attachment_point.expect("FABRIK constraint expects a parent");

        let Some(first) = joints.first() else {
            return;
        };

        let Some(last) = joints.last() else {
            return;
        };

        if self.current_target_position.distance(last.pos) > self.max_distance
            || first.pos.distance(last.pos) > self.target_distance
        {
            self.current_target_position = attachment_point.position
                + self.target_distance
                    * Vec2::from_angle(attachment_point.angle + self.target_angle);
        }
        let target_position = self.current_target_position;

        let Some(last) = joints.last_mut() else {
            return;
        };
        last.pos = target_position;

        self.distance_constraint.direction = Direction::Backwards;
        self.distance_constraint.apply(joints, None);

        if let Some(first) = joints.first_mut() {
            first.pos = attachment_point.position;
        }

        self.distance_constraint.direction = Direction::Forward;
        self.distance_constraint.apply(joints, None);
    }
}
