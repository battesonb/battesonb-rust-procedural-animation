use std::f32::consts::PI;

use itertools::Itertools;
use lending_iterator::prelude::*;
use macroquad::prelude::*;

use crate::{
    constants::{DEBUG, DEBUG_LINE_THICKNESS, LINE_THICKNESS},
    constraints::Constraint,
    joint::Joint,
};

#[derive(Debug)]
pub struct Body {
    pub joints: Vec<Joint>,
    pub constraints: Vec<Box<dyn Constraint>>,
}

impl Body {
    pub fn new(joints: Vec<Joint>) -> Self {
        Self {
            joints,
            constraints: Vec::new(),
        }
    }

    pub fn with_constraint(mut self, constraint: impl Constraint + 'static) -> Self {
        self.add_constraint(constraint);
        self
    }

    pub fn add_constraint(&mut self, constraint: impl Constraint + 'static) {
        self.constraints.push(Box::new(constraint));
    }

    pub fn apply_constraints(&mut self) {
        for constrait in &self.constraints {
            constrait.apply(&mut self.joints);
        }

        // update joint angles
        let mut iter = self.joints.windows_mut::<2>();
        while let Some([a, b]) = iter.next() {
            b.angle = (b.pos - a.pos).normalize().to_angle();
        }

        // Make the first angle the same as the second, since it's skipped above
        let mut iter = self.joints.iter_mut();
        let Some(first) = iter.next() else {
            return;
        };
        let Some(second) = iter.next() else {
            return;
        };

        first.angle = second.angle;
    }

    /// Produces a zig-zag of points. This is useful for tessellation, but a bit painful for line
    /// drawing. Trade-offs!
    fn points(&self) -> Vec<Vec2> {
        const END_STEPS: usize = 4;

        let first = self.joints.first();
        let front_points = first.iter().flat_map(|joint| {
            (0..END_STEPS).flat_map(|i| {
                let angle = (i as f32 / END_STEPS as f32) * PI / 2.0;
                vec![
                    joint.pos + joint.radius * Vec2::from_angle(joint.angle + PI + angle),
                    joint.pos + joint.radius * Vec2::from_angle(joint.angle + PI - angle),
                ]
            })
        });

        let last = self.joints.last();
        let back_points = last.iter().flat_map(|joint| {
            (0..END_STEPS).rev().flat_map(|i| {
                let angle = (i as f32 / END_STEPS as f32) * PI / 2.0;
                vec![
                    joint.pos + joint.radius * Vec2::from_angle(joint.angle - angle),
                    joint.pos + joint.radius * Vec2::from_angle(joint.angle + angle),
                ]
            })
        });

        let left_points = self
            .joints
            .iter()
            .map(|joint| joint.pos + joint.radius * Vec2::from_angle(joint.angle + PI / 2.0));

        let interleaved_points = self
            .joints
            .iter()
            .map(|joint| joint.pos + joint.radius * Vec2::from_angle(joint.angle - PI / 2.0))
            .interleave(left_points);

        front_points
            .chain(interleaved_points)
            .chain(back_points)
            .collect::<Vec<_>>()
    }

    pub fn draw(&self) {
        let points = self.points();

        for (a, b) in points.iter().step_by(2).tuple_windows() {
            draw_line(a.x, a.y, b.x, b.y, LINE_THICKNESS, BLACK);
        }

        for (a, b) in points.iter().skip(1).step_by(2).tuple_windows() {
            draw_line(a.x, a.y, b.x, b.y, LINE_THICKNESS, BLACK);
        }

        if DEBUG {
            for circle in &self.joints {
                let pos = circle.pos;
                draw_circle_lines(pos.x, pos.y, circle.radius, DEBUG_LINE_THICKNESS, BLUE);
            }
        }
    }
}
