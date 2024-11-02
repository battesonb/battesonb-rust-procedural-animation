use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::joint::Joint;

pub trait Constraint: Debug + DynClone {
    fn apply(&self, joints: &mut Vec<Joint>);
}

dyn_clone::clone_trait_object!(Constraint);
