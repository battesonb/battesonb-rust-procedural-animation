mod body;
mod constants;
mod constraints;
mod joint;

use std::f32::consts::PI;

use body::Body;
use constraints::{AngleConstraint, DistanceConstraint};
use joint::Joint;
use macroquad::{camera::set_camera, prelude::*};

#[macroquad::main("Procedural Animation")]
async fn main() {
    let total_joints = 10;
    let mut body = Body::new(
        (0..total_joints)
            .map(|i| {
                Joint::new(
                    0.0,
                    20.0 * i as f32,
                    30.0 + 10.0 * (2.0 * PI * (i as f32 / total_joints as f32)).sin(),
                )
            })
            .collect::<Vec<_>>(),
    )
    .with_constraint(DistanceConstraint::new(30.0, 1.0))
    .with_constraint(AngleConstraint::new(0.75 * PI, 0.5));

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
