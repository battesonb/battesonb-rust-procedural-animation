mod body;
mod constants;
mod constraints;
mod extensions;
mod joint;

use crate::body::Side;
use std::{
    collections::HashSet,
    f32::consts::PI,
    sync::atomic::{AtomicU64, Ordering},
};

use body::{Body, BodyDescriptor};
use constants::BACKGROUND_COLOR;
use constraints::{
    AngleConstraintDescriptor, ConstraintDescriptor, DistanceConstraintDescriptor,
    FabrikConstraintDescriptor,
};
use extensions::{ColorExtension, UiExtension, UI_WIDTH};
use joint::JointDescriptor;
use macroquad::{
    camera::set_camera,
    prelude::*,
    ui::{
        hash, root_ui,
        widgets::{self},
        Skin,
    },
};

#[derive(Clone, Debug, PartialEq)]
struct BodyConfiguration {
    angle_constraint: f32,
    radius: f32,
    shapes: Vec<BodyShape>,
    joints: f32,
    joint_distance: f32,
    legs: Vec<LegConfiguration>,
    color: Color,
}

#[derive(Clone, Debug, PartialEq)]
struct BodyShape {
    id: u64,
    amplitude: f32,
    constant_offset: f32,
    frequency_multiplier: f32,
}

pub static UI_ID: AtomicU64 = AtomicU64::new(1);

impl BodyShape {
    fn random() -> BodyShape {
        Self {
            amplitude: rand::RandomRange::gen_range(0., 30.),
            constant_offset: rand::RandomRange::gen_range(0., 2. * PI),
            frequency_multiplier: rand::RandomRange::gen_range(-12., 12.),
            ..Default::default()
        }
    }
}

impl Default for BodyShape {
    fn default() -> Self {
        Self {
            id: UI_ID.fetch_add(1, Ordering::Relaxed),
            amplitude: 5.,
            constant_offset: 0.,
            frequency_multiplier: 1.,
        }
    }
}

impl BodyConfiguration {
    fn sanitize(&mut self) {
        self.joints = self.joints.ceil();

        for leg in &mut self.legs {
            leg.joints = leg.joints.ceil();
        }
    }
}

impl Default for BodyConfiguration {
    fn default() -> Self {
        Self {
            angle_constraint: 0.9 * PI,
            color: Color::from_hex(0x61A5B8),
            radius: 30.,
            joints: 20.,
            joint_distance: 20.,
            shapes: vec![
                BodyShape {
                    amplitude: 10.,
                    constant_offset: 0.,
                    frequency_multiplier: 2. * PI,
                    ..Default::default()
                },
                BodyShape {
                    amplitude: 12.,
                    constant_offset: 1.8,
                    frequency_multiplier: -1.2 * PI,
                    ..Default::default()
                },
                BodyShape {
                    amplitude: 3.,
                    constant_offset: 4.3,
                    frequency_multiplier: PI,
                    ..Default::default()
                },
            ],
            legs: vec![
                LegConfiguration {
                    joint_distance: 30.,
                    body_ratio: 0.25,
                    ..Default::default()
                },
                LegConfiguration {
                    joint_distance: 25.,
                    body_ratio: 0.6,
                    ..Default::default()
                },
            ],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct LegConfiguration {
    id: u64,
    angle: f32,
    joints: f32,
    joint_distance: f32,
    body_ratio: f32,
    target_ratio: f32,
    target_max_distance: f32,
    thickness: f32,
}

impl LegConfiguration {
    fn random() -> Self {
        Self {
            thickness: rand::RandomRange::gen_range(5., 25.),
            angle: PI * rand::RandomRange::gen_range(0.5, 0.8),
            joints: 3.,
            joint_distance: rand::RandomRange::gen_range(20., 30.),
            body_ratio: rand::RandomRange::gen_range(0., 1.),
            target_ratio: rand::RandomRange::gen_range(0.4, 0.8),
            target_max_distance: rand::RandomRange::gen_range(60., 80.),
            ..Default::default()
        }
    }
}

impl Default for LegConfiguration {
    fn default() -> Self {
        Self {
            id: UI_ID.fetch_add(1, Ordering::Relaxed),
            thickness: 12.,
            angle: PI * 0.75,
            joints: 3.,
            joint_distance: 25.,
            body_ratio: 0.4,
            target_ratio: 0.65,
            target_max_distance: 100.,
        }
    }
}

#[macroquad::main("Procedural Animation")]
async fn main() {
    configure_ui_skin();

    let mut body_config = BodyConfiguration::default();
    let mut last_body_config = body_config.clone();

    let camera = Camera2D::from_display_rect(Rect::new(
        -screen_width() / 2.0,
        -screen_height() / 2.0,
        screen_width(),
        screen_height(),
    ));

    set_camera(&camera);

    let mut use_mouse = false;
    let mut debug = false;

    let mut body = build_body(&body_config);
    loop {
        // update
        body.apply_constraints(None);

        if is_mouse_button_pressed(MouseButton::Left)
            && !root_ui().is_mouse_over(mouse_position().into())
        {
            use_mouse = !use_mouse;
        }

        let (x, y) = mouse_position();
        let mouse_world = camera.screen_to_world(Vec2::new(x, y));
        if let Some(first) = body.joints.first_mut() {
            if use_mouse {
                first.pos = first.pos.lerp(mouse_world, 0.1);
            } else {
                let x = get_time().cos() as f32;
                let y = (get_time() * 2.).sin() as f32;
                first.pos = first.pos.lerp(
                    (Vec2::new(x, y)
                        * (camera.screen_to_world(Vec2::new(screen_width(), screen_height()))))
                        / 2.,
                    0.1,
                );
            };
        }

        // draw
        clear_background(BACKGROUND_COLOR);
        body.draw();

        if debug {
            body.debug_draw();
        }

        widgets::Window::new(
            hash!(),
            Vec2::new(5., 5.),
            Vec2::new(UI_WIDTH, screen_height() - 10.),
        )
        .label("Configuration")
        .titlebar(true)
        .ui(&mut root_ui(), |ui| {
            if ui.button(None, format!("Debug: {}", debug)) {
                debug = !debug;
            }
            ui.label(None, "Body");
            ui.color(hash!(), "Color", &mut body_config.color);
            ui.slider(hash!(), "Radius", 1.0..50.0, &mut body_config.radius);
            ui.slider(
                hash!(),
                "Max Angle",
                (PI / 2.)..PI,
                &mut body_config.angle_constraint,
            );
            ui.slider(hash!(), "Joints", 1.0..50.0, &mut body_config.joints);
            ui.slider(
                hash!(),
                "Joint distance",
                1.0..50.0,
                &mut body_config.joint_distance,
            );
            ui.rule();
            ui.tree_node(hash!(), "Shaping", |ui| {
                if ui.button(None, "Add") {
                    body_config.shapes.push(BodyShape::random());
                }
                let mut marked_for_deletion = HashSet::new();
                for shape in &mut body_config.shapes {
                    ui.slider(
                        hash!("shape.amplitude", shape.id),
                        "Amplitude",
                        0.0..30.,
                        &mut shape.amplitude,
                    );
                    ui.slider(
                        hash!("shape.constant_offset", shape.id),
                        "Offset",
                        0.0..(2. * PI),
                        &mut shape.constant_offset,
                    );
                    ui.slider(
                        hash!("shape.frequency_multiplier", shape.id),
                        "Frequency multiplier",
                        -30.0..30.,
                        &mut shape.frequency_multiplier,
                    );
                    if ui.button(None, "Delete") {
                        marked_for_deletion.insert(shape.id);
                    }
                }
                if !marked_for_deletion.is_empty() {
                    body_config
                        .shapes
                        .retain(|shape| !marked_for_deletion.contains(&shape.id));
                }
            });
            ui.rule();
            ui.tree_node(hash!(), "Legs", |ui| {
                if ui.button(None, "Add") {
                    body_config.legs.push(LegConfiguration::random());
                }
                ui.separator();
                let mut marked_for_deletion = HashSet::new();
                let mut clones = Vec::new();
                for leg in &mut body_config.legs {
                    ui.slider(
                        hash!("leg.thickness", leg.id),
                        "Thickness",
                        1.0..30.,
                        &mut leg.thickness,
                    );
                    ui.slider(
                        hash!("leg.angle", leg.id),
                        "Angle",
                        0.0..(2. * PI),
                        &mut leg.angle,
                    );
                    ui.slider(
                        hash!("leg.joints", leg.id),
                        "Joints",
                        2.0..10.0,
                        &mut leg.joints,
                    );
                    ui.slider(
                        hash!("leg.joint_distance", leg.id),
                        "Joint distance",
                        1.0..50.0,
                        &mut leg.joint_distance,
                    );
                    ui.slider(
                        hash!("leg.target_ratio", leg.id),
                        "Target ratio",
                        0.0..1.0,
                        &mut leg.target_ratio,
                    );
                    ui.slider(
                        hash!("leg.target_max_distance", leg.id),
                        "Max target distance",
                        1.0..200.0,
                        &mut leg.target_max_distance,
                    );
                    ui.slider(
                        hash!("leg.body_ratio", leg.id),
                        "Body ratio",
                        0.0..1.0,
                        &mut leg.body_ratio,
                    );
                    if ui.button(None, "Duplicate") {
                        let mut leg_duplicate = leg.clone();
                        leg_duplicate.id = UI_ID.fetch_add(1, Ordering::Relaxed);
                        clones.push(leg_duplicate);
                    }
                    if ui.button(None, "Delete") {
                        marked_for_deletion.insert(leg.id);
                    }
                }
                ui.separator();
                body_config.legs.extend(clones);
                if !marked_for_deletion.is_empty() {
                    body_config
                        .legs
                        .retain(|leg| !marked_for_deletion.contains(&leg.id));
                }
            });
        });

        // Sanitize UI state
        body_config.sanitize();

        if body_config != last_body_config {
            body = build_body(&body_config);
        }

        last_body_config = body_config.clone();

        next_frame().await;
    }
}

fn build_body(body_config: &BodyConfiguration) -> Body {
    let total_joints = body_config.joints as usize;
    let mut body = BodyDescriptor {
        fill_color: body_config.color,
        joints: (0..total_joints)
            .map(|i| {
                let radius = body_config.radius
                    + body_config
                        .shapes
                        .iter()
                        .map(|s| {
                            s.amplitude
                                * (s.constant_offset
                                    + s.frequency_multiplier * (i as f32 / total_joints as f32))
                                    .sin()
                        })
                        .sum::<f32>();
                JointDescriptor {
                    radius: radius.max(1.),
                    ..Default::default()
                }
            })
            .collect::<Vec<_>>(),
        constraints: vec![
            ConstraintDescriptor::Distance(DistanceConstraintDescriptor {
                distance: body_config.joint_distance,
                ..Default::default()
            }),
            ConstraintDescriptor::Angle(AngleConstraintDescriptor {
                angle: body_config.angle_constraint,
                rate: 0.5,
            }),
        ],
        ..Default::default()
    };

    // eyes
    let head = body.joints.first_mut().unwrap();
    for mult in [-1., 1.] {
        let eye = JointDescriptor {
            radius: 12.,
            bodies: vec![
                BodyDescriptor {
                    line_thickness: 0.,
                    joints: vec![JointDescriptor {
                        radius: 5.,
                        ..Default::default()
                    }],
                    attachment_angle: mult * -PI * 0.45,
                    attachment_offset: 0.3,
                    ..Default::default()
                },
                BodyDescriptor {
                    line_thickness: 0.,
                    joints: vec![JointDescriptor {
                        radius: 3.5,
                        ..Default::default()
                    }],
                    attachment_angle: mult * PI * 0.2,
                    attachment_offset: 0.5,
                    ..Default::default()
                },
            ],
        };

        head.add_body(BodyDescriptor {
            line_color: WHITE,
            line_thickness: 5.,
            fill_color: Color::from_hex(0x704e37),
            joints: vec![eye],
            attachment_angle: mult * PI * 0.7,
            attachment_offset: 0.7,
            ..Default::default()
        });
    }

    for leg in &body_config.legs {
        let joint_index = (body.joints.len() as f32 * leg.body_ratio) as usize;
        if let Some(joint) = body.joints.get_mut(joint_index) {
            for mult in [-1., 1.] {
                joint.add_body(BodyDescriptor {
                    fill_color: body_config.color.mul(Color::from_hex(0xDDDDDD)),
                    joints: (0..(leg.joints as usize))
                        .map(|_| JointDescriptor {
                            radius: leg.thickness,
                            ..Default::default()
                        })
                        .collect::<Vec<_>>(),
                    attachment_angle: mult * PI / 2.0,
                    attachment_offset: (joint.radius - leg.thickness).max(0.) / joint.radius,
                    side: Side::Back,
                    constraints: vec![ConstraintDescriptor::Fabrik(FabrikConstraintDescriptor {
                        joint_distance: leg.joint_distance,
                        target_angle: mult * leg.angle,
                        target_distance: (leg.joint_distance * leg.joints) * leg.target_ratio,
                        max_distance: leg.target_max_distance,
                        ..Default::default()
                    })],
                    ..Default::default()
                });
            }
        }
    }

    return Body::new(body);
}

fn configure_ui_skin() {
    let window_style = root_ui()
        .style_builder()
        .background(Image::gen_image_color(
            1,
            1,
            Color::from_rgba(200, 200, 200, 100),
        ))
        .build();

    let skin = Skin {
        window_style,
        ..root_ui().default_skin()
    };

    root_ui().push_skin(&skin);
}
