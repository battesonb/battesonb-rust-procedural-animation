use std::f32::consts::PI;

use itertools::Itertools;
use lending_iterator::prelude::*;
use macroquad::prelude::*;

use crate::{
    constants::{DEBUG, DEBUG_LINE_THICKNESS, LINE_COLOR, LINE_THICKNESS},
    constraints::Constraint,
    joint::Joint,
};

#[derive(Clone, Debug)]
pub struct Body {
    pub color: Color,
    pub joints: Vec<Joint>,
    pub constraints: Vec<Box<dyn Constraint>>,
    pub attachment_angle: f32,
    pub attachment_offset: f32,
}

impl Body {
    pub fn new(color: Color, joints: Vec<Joint>) -> Self {
        Self {
            color,
            joints,
            constraints: Vec::new(),
            attachment_angle: 0.0,
            attachment_offset: 0.0,
        }
    }

    pub fn with_attachment_angle(mut self, angle: f32) -> Self {
        self.attachment_angle = angle;
        self
    }

    pub fn with_attachment_offset(mut self, offset: f32) -> Self {
        self.attachment_offset = offset;
        self
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

        // Apply constraints to inner bodies, first of which is that the first joint of a body is
        // always fixed to the body's parent joint.
        for joint in &mut self.joints {
            for body in &mut joint.bodies {
                if let Some(first_joint) = body.joints.first_mut() {
                    first_joint.pos = joint.pos
                        + body.attachment_offset
                            * joint.radius
                            * Vec2::from_angle(joint.angle + body.attachment_angle);
                }
                body.apply_constraints();
            }
        }
    }

    /// Produces a zig-zag of points. This is useful for tessellation, but a bit painful for line
    /// drawing. Trade-offs!
    fn points(&self) -> Vec<Vec2> {
        const END_STEPS: usize = 4;

        let first = self.joints.first();
        let front_points = first.iter().flat_map(|joint| {
            (0..END_STEPS).flat_map(|i| {
                let angle = (i as f32 / END_STEPS as f32) * (PI * 0.45 + 0.25);
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

    fn mesh<'a, T>(&self, points: T) -> Mesh
    where
        T: IntoIterator<Item = &'a Vec2>,
    {
        let vertices = points
            .into_iter()
            .map(|point| Vertex::new2(point.extend(0.0), Vec2::ZERO, self.color))
            .collect::<Vec<_>>();
        let index_count = (vertices.len() - 2).max(0) / 2;
        let indices = (0..index_count)
            .flat_map(|i| {
                let s = (i as u16) * 2;
                [s, s + 1, s + 2, s + 1, s + 2, s + 3]
            })
            .collect::<Vec<_>>();

        Mesh {
            vertices,
            indices,
            texture: None,
        }
    }

    pub fn draw(&self) {
        let points = self.points();

        for (a, b) in points.iter().step_by(2).tuple_windows() {
            draw_line(a.x, a.y, b.x, b.y, LINE_THICKNESS, LINE_COLOR);
        }

        for (a, b) in points.iter().skip(1).step_by(2).tuple_windows() {
            draw_line(a.x, a.y, b.x, b.y, LINE_THICKNESS, LINE_COLOR);
        }

        let mesh = self.mesh(&points);
        draw_mesh(&mesh);

        for joint in &self.joints {
            joint.draw();
        }

        if DEBUG {
            for circle in &self.joints {
                let pos = circle.pos;
                draw_circle_lines(pos.x, pos.y, circle.radius, DEBUG_LINE_THICKNESS, BLUE);
            }
        }
    }
}
