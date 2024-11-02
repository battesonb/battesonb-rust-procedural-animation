use crate::joint::Joint;

use super::Constraint;
use lending_iterator::prelude::*;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct DistanceConstraint {
    distance: f32,
    rate: f32,
}

impl DistanceConstraint {
    pub fn new(distance: f32, rate: f32) -> Self {
        Self { distance, rate }
    }

    #[inline]
    pub fn distance_squared(&self) -> f32 {
        self.distance * self.distance
    }
}

impl Constraint for DistanceConstraint {
    fn apply(&self, joints: &mut Vec<Joint>) {
        let mut iter = joints.windows_mut::<2>();
        while let Some([a, b]) = iter.next() {
            let delta = b.pos - a.pos;
            let distance_squared = delta.length_squared();
            if distance_squared <= self.distance_squared() {
                continue;
            }

            let distance = distance_squared.sqrt();
            let target = a.pos + self.distance * delta / distance;

            b.pos = b.pos.lerp(target, self.rate);
        }
    }
}
