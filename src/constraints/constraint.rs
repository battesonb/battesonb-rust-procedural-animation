use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::{body::AttachmentPoint, joint::Joint};

use super::{
    AngleConstraint, AngleConstraintDescriptor, DistanceConstraint, DistanceConstraintDescriptor,
    FabrikConstraint, FabrikConstraintDescriptor,
};

pub trait Constraint: Debug + DynClone {
    fn apply(&mut self, joints: &mut Vec<Joint>, attachment_point: Option<AttachmentPoint>);
}

dyn_clone::clone_trait_object!(Constraint);

#[derive(Clone, Debug)]
pub enum ConstraintDescriptor {
    Distance(DistanceConstraintDescriptor),
    Angle(AngleConstraintDescriptor),
    Fabrik(FabrikConstraintDescriptor),
}

impl From<ConstraintDescriptor> for Box<dyn Constraint> {
    fn from(descriptor: ConstraintDescriptor) -> Self {
        match descriptor {
            ConstraintDescriptor::Distance(descriptor) => {
                Box::new(DistanceConstraint::new(descriptor))
            }
            ConstraintDescriptor::Angle(descriptor) => Box::new(AngleConstraint::new(descriptor)),
            ConstraintDescriptor::Fabrik(descriptor) => Box::new(FabrikConstraint::new(descriptor)),
        }
    }
}
