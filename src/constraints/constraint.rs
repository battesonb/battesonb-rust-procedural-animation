use std::fmt::Debug;

use crate::joint::Joint;

pub trait Constraint: Debug {
    fn apply(&self, joints: &mut Vec<Joint>);
}
