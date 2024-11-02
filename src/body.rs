use std::f32::consts::PI;

use itertools::Itertools;
use lending_iterator::prelude::*;
use macroquad::prelude::*;

use crate::{
    constants::{DEBUG_COLOR, DEBUG_LINE_THICKNESS},
    constraints::{Constraint, ConstraintDescriptor},
    joint::{Joint, JointDescriptor},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Side {
    #[default]
    Front,
    Back,
}

#[derive(Clone, Debug)]
pub struct Body {
    pub line_color: Color,
    pub line_thickness: f32,
    pub fill_color: Color,
    pub joints: Vec<Joint>,
    pub constraints: Vec<Box<dyn Constraint>>,
    pub attachment_angle: f32,
    pub attachment_offset: f32,
    pub side: Side,
}

#[derive(Copy, Clone, Debug)]
pub struct AttachmentPoint {
    pub position: Vec2,
    pub angle: f32,
}

impl Body {
    pub fn new(descriptor: BodyDescriptor) -> Self {
        let BodyDescriptor {
            line_color,
            line_thickness,
            fill_color,
            joints,
            constraints,
            attachment_angle,
            attachment_offset,
            side,
        } = descriptor;

        Self {
            line_color,
            line_thickness,
            fill_color,
            joints: joints.into_iter().map(Into::into).collect::<Vec<_>>(),
            constraints: constraints.into_iter().map(Into::into).collect::<Vec<_>>(),
            attachment_angle,
            attachment_offset,
            side,
        }
    }

    pub fn with_constraint(mut self, constraint: impl Constraint + 'static) -> Self {
        self.add_constraint(constraint);
        self
    }

    pub fn add_constraint(&mut self, constraint: impl Constraint + 'static) {
        self.constraints.push(Box::new(constraint));
    }

    pub fn apply_constraints(&mut self, attachment_point: Option<AttachmentPoint>) {
        for constrait in &mut self.constraints {
            constrait.apply(&mut self.joints, attachment_point);
        }

        // update joint angles
        let mut iter = self.joints.windows_mut::<2>();
        while let Some([a, b]) = iter.next() {
            b.angle = (b.pos - a.pos).normalize_or(Vec2::X).to_angle();
        }

        // Make the first angle the same as the second, since it's skipped above
        'angle: {
            let mut iter = self.joints.iter_mut();
            let Some(first) = iter.next() else {
                break 'angle;
            };
            let Some(second) = iter.next() else {
                break 'angle;
            };

            first.angle = second.angle;
        }

        // Apply constraints to inner bodies, first of which is that the first joint of a body is
        // always fixed to the body's parent joint.
        let angle = attachment_point.map_or(0., |ap| ap.angle);
        for joint in &mut self.joints {
            for body in &mut joint.bodies {
                let attachment_point = AttachmentPoint {
                    position: joint.pos
                        + body.attachment_offset
                            * joint.radius
                            * Vec2::from_angle(joint.angle + body.attachment_angle + angle),
                    angle: self.attachment_angle + joint.angle + angle,
                };
                if let Some(first_joint) = body.joints.first_mut() {
                    first_joint.pos = attachment_point.position;
                }
                body.apply_constraints(Some(attachment_point));
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
            .map(|point| Vertex::new2(point.extend(0.0), Vec2::ZERO, self.fill_color))
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

    pub fn draw(&self, debug: bool) {
        for joint in &self.joints {
            joint.draw(Side::Back, debug);
        }

        let points = self.points();

        if self.line_thickness > 0. {
            for (a, b) in points
                .iter()
                .step_by(2)
                .tuple_windows()
                .chain(points.iter().skip(1).step_by(2).tuple_windows())
            {
                draw_circle(a.x, a.y, self.line_thickness / 2., self.line_color);
                draw_line(a.x, a.y, b.x, b.y, self.line_thickness, self.line_color);
                draw_circle(b.x, b.y, self.line_thickness / 2., self.line_color);
            }
        }

        let mesh = self.mesh(&points);
        draw_mesh(&mesh);

        for joint in &self.joints {
            joint.draw(Side::Front, debug);
        }

        if debug {
            for circle in &self.joints {
                let pos = circle.pos;
                draw_circle_lines(
                    pos.x,
                    pos.y,
                    circle.radius,
                    DEBUG_LINE_THICKNESS,
                    DEBUG_COLOR,
                );
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct BodyDescriptor {
    pub line_color: Color,
    pub line_thickness: f32,
    pub fill_color: Color,
    pub joints: Vec<JointDescriptor>,
    pub constraints: Vec<ConstraintDescriptor>,
    pub attachment_angle: f32,
    pub attachment_offset: f32,
    pub side: Side,
}

impl Default for BodyDescriptor {
    fn default() -> Self {
        Self {
            line_color: BLACK,
            line_thickness: 6.0,
            fill_color: WHITE,
            joints: Vec::new(),
            constraints: Vec::new(),
            attachment_angle: 0.0,
            attachment_offset: 0.0,
            side: Side::Front,
        }
    }
}

impl BodyDescriptor {
    pub fn with_constraint(mut self, constraint: ConstraintDescriptor) -> Self {
        self.add_constraint(constraint);
        self
    }

    pub fn add_constraint(&mut self, constraint: ConstraintDescriptor) {
        self.constraints.push(constraint);
    }
}

impl From<BodyDescriptor> for Body {
    fn from(descriptor: BodyDescriptor) -> Self {
        Self::new(descriptor)
    }
}
