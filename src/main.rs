mod body;
mod constants;
mod constraints;
mod joint;

use crate::body::Side;
use crate::joint::JointDescriptor;
use std::f32::consts::PI;

use body::{Body, BodyDescriptor};
use constants::BACKGROUND_COLOR;
use constraints::{
    AngleConstraintDescriptor, ConstraintDescriptor, DistanceConstraintDescriptor,
    FabrikConstraintDescriptor,
};
use macroquad::{camera::set_camera, prelude::*};

#[macroquad::main("Procedural Animation")]
async fn main() {
    let total_joints = 25;
    let mut body = BodyDescriptor {
        fill_color: Color::from_hex(0x61A5B8),
        joints: (0..total_joints)
            .map(|i| {
                let s1 = 10.0 * (2.0 * PI * (i as f32 / total_joints as f32)).sin();
                let c1 = 12.0 * (0.2 - 1.2 * PI * (i as f32 / total_joints as f32)).cos();
                let c2 = 3.0 * (10.0 * PI * (i as f32 / total_joints as f32)).cos();
                JointDescriptor {
                    radius: 30.0 + s1 + c1 + c2,
                    ..Default::default()
                }
            })
            .collect::<Vec<_>>(),
        constraints: vec![
            ConstraintDescriptor::Distance(DistanceConstraintDescriptor {
                distance: 15.0,
                ..Default::default()
            }),
            ConstraintDescriptor::Angle(AngleConstraintDescriptor {
                angle: 0.9 * PI,
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
            line_thickness: 3.,
            fill_color: Color::from_hex(0x704e37),
            joints: vec![eye],
            attachment_angle: mult * PI * 0.7,
            attachment_offset: 0.7,
            ..Default::default()
        });
    }

    // legs
    let shoulders = &mut body.joints[5];
    for mult in [-1., 1.] {
        shoulders.add_body(BodyDescriptor {
            fill_color: Color::from_hex(0x588BAD),
            joints: (0..3)
                .map(|i| {
                    let s1 = 4.0 * (2.0 * PI * (i as f32 / 10.0)).sin();
                    let c1 = 2.0 * (0.2 - 1.2 * PI * (i as f32 / 10.0)).cos();
                    JointDescriptor {
                        radius: 7. + s1 + c1,
                        ..Default::default()
                    }
                })
                .collect::<Vec<_>>(),
            attachment_angle: mult * PI / 2.0,
            attachment_offset: 1.0,
            side: Side::Back,
            constraints: vec![ConstraintDescriptor::Fabrik(FabrikConstraintDescriptor {
                joint_distance: 30.,
                target_angle: mult * PI * 0.75,
                target_distance: 50.,
                max_distance: 10.0,
                ..Default::default()
            })],
            ..Default::default()
        });
    }

    let hips = &mut body.joints[14];
    for mult in [-1., 1.] {
        hips.add_body(BodyDescriptor {
            fill_color: Color::from_hex(0x588BAD),
            joints: (0..3)
                .map(|i| {
                    let s1 = 4.0 * (2.0 * PI * (i as f32 / 10.0)).sin();
                    let c1 = 2.0 * (0.2 - 1.2 * PI * (i as f32 / 10.0)).cos();
                    JointDescriptor {
                        radius: 7.0 + s1 + c1,
                        ..Default::default()
                    }
                })
                .collect::<Vec<_>>(),
            attachment_angle: mult * PI / 2.0,
            attachment_offset: 1.0,
            side: Side::Back,
            constraints: vec![ConstraintDescriptor::Fabrik(FabrikConstraintDescriptor {
                joint_distance: 30.,
                target_angle: mult * PI * 0.65,
                target_distance: 45.,
                max_distance: 10.0,
                ..Default::default()
            })],
            ..Default::default()
        });
    }

    let camera = Camera2D::from_display_rect(Rect::new(
        -screen_width() / 2.0,
        -screen_height() / 2.0,
        screen_width(),
        screen_height(),
    ));

    set_camera(&camera);

    let mut use_mouse = false;
    let mut debug = false;

    let mut body = Body::new(body);
    loop {
        // update
        body.apply_constraints(None);

        if is_mouse_button_pressed(MouseButton::Left) {
            use_mouse = !use_mouse;
        }

        if is_key_pressed(KeyCode::D) {
            debug = !debug;
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
        body.draw(debug);

        next_frame().await;
    }
}
