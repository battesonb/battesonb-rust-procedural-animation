mod body;
mod constants;
mod constraints;
mod joint;

use std::f32::consts::PI;

use body::Body;
use constants::BACKGROUND_COLOR;
use constraints::{AngleConstraint, DistanceConstraint};
use joint::Joint;
use macroquad::{camera::set_camera, prelude::*};
use rand::RandomRange;

#[macroquad::main("Procedural Animation")]
async fn main() {
    let total_joints = 25;
    let mut body = Body::new(
        Color::from_hex(0x61A5B8),
        (0..total_joints)
            .map(|i| {
                let s1 = 10.0 * (2.0 * PI * (i as f32 / total_joints as f32)).sin();
                let c1 = 12.0 * (0.2 - 1.2 * PI * (i as f32 / total_joints as f32)).cos();
                let c2 = 3.0 * (10.0 * PI * (i as f32 / total_joints as f32)).cos();
                Joint::new(
                    RandomRange::gen_range(0.0, screen_width()),
                    RandomRange::gen_range(0.0, screen_height()),
                    30.0 + s1 + c1 + c2,
                )
            })
            .collect::<Vec<_>>(),
    )
    .with_constraint(DistanceConstraint::new(15.0, 1.0))
    .with_constraint(AngleConstraint::new(0.75 * PI, 0.5));

    // eyes
    let head = body.joints.first_mut().unwrap();
    head.add_body(
        Body::new(BLACK, vec![Joint::new(0.0, 0.0, 5.0)])
            .with_attachment_angle(PI * 0.7)
            .with_attachment_offset(0.75),
    );
    head.add_body(
        Body::new(BLACK, vec![Joint::new(0.0, 0.0, 5.0)])
            .with_attachment_angle(-PI * 0.7)
            .with_attachment_offset(0.75),
    );

    let camera = Camera2D::from_display_rect(Rect::new(
        -screen_width() / 2.0,
        -screen_height() / 2.0,
        screen_width(),
        screen_height(),
    ));

    set_camera(&camera);

    let mut use_mouse = false;
    loop {
        // update
        body.apply_constraints();

        if is_mouse_button_pressed(MouseButton::Left) {
            use_mouse = !use_mouse;
        }

        let (x, y) = mouse_position();
        let mouse_world = camera.screen_to_world(Vec2::new(x, y));
        if let Some(first) = body.joints.first_mut() {
            if use_mouse {
                first.pos = mouse_world;
            } else {
                first.pos = (screen_height().min(screen_width()) - 200.0) / 2.0
                    * Vec2::from_angle(get_time() as f32);
            };
        }

        // draw
        clear_background(WHITE);

        body.draw();

        next_frame().await
    }
}
