use macroquad::{
    color::{Color, GRAY},
    math::Vec2,
    ui::{hash, Id, Ui},
};

pub trait ColorExtension {
    fn mul(&self, rhs: Color) -> Color;
    fn add(&self, rhs: Color) -> Color;
}

pub trait ColorScalarExtension {
    fn mul(&self, rhs: Color) -> Color;
}

impl ColorScalarExtension for f32 {
    fn mul(&self, rhs: Color) -> Color {
        Color::new(self * rhs.r, self * rhs.g, self * rhs.b, rhs.a)
    }
}

impl ColorExtension for Color {
    fn add(&self, rhs: Color) -> Color {
        Color::new(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a.max(rhs.a),
        )
    }

    fn mul(&self, rhs: Color) -> Color {
        Color::new(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b,
            self.a * rhs.a,
        )
    }
}

pub const UI_WIDTH: f32 = 250.;
pub const PADDING: f32 = 5.;

pub trait UiExtension {
    fn color(&mut self, id: Id, label: &str, data: &mut Color);
    fn rule(&mut self);
}

impl UiExtension for Ui {
    fn color(&mut self, id: Id, label: &str, data: &mut Color) {
        self.separator();
        self.group(id, Vec2::new(UI_WIDTH - PADDING * 2., 100.), |ui| {
            ui.label(None, label);
            ui.slider(hash!("r", id), "Red", 0.0..1., &mut data.r);
            ui.slider(hash!("g", id), "Green", 0.0..1., &mut data.g);
            ui.slider(hash!("b", id), "Blue", 0.0..1., &mut data.b);
        });
    }

    fn rule(&mut self) {
        let mut canvas = self.canvas();
        canvas.request_space(Vec2::new(UI_WIDTH - PADDING * 2., 10.));
        let cursor = canvas.cursor() + Vec2::new(-PADDING * 2., 7.);
        canvas.line(cursor.with_x(0.0), cursor, GRAY);
    }
}
