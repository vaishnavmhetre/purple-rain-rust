use nannou::prelude::*;
use rand::{thread_rng, Rng};

use crate::utils::window;

pub struct Space {
    window_id: window::Id,

    drops: Vec<Drop>,
}

impl Space {
    pub fn new(window_id: window::Id, drops: Vec<Drop>) -> Self {
        Self { window_id, drops }
    }

    pub fn window_id(&self) -> WindowId {
        self.window_id
    }

    pub fn set_window_id(&mut self, window_id: window::Id) {
        self.window_id = window_id;
    }

    pub fn drops(&self) -> &[Drop] {
        self.drops.as_ref()
    }

    pub fn drops_mut(&mut self) -> &mut Vec<Drop> {
        &mut self.drops
    }

    pub fn set_drops(&mut self, drops: Vec<Drop>) {
        self.drops = drops;
    }

    pub fn update(&mut self, app: &App, update: Update) {
        let window_id = self.window_id();
        self.drops_mut().into_iter().for_each(|s| {
            s.update(app, update, window_id);
        });
    }

    pub fn draw(&self, app: &App, frame: Frame) {
        let draw = app.draw();
        draw.background().color(rgb8(230, 230, 250));

        self.drops().iter().for_each(|s| {
            s.draw(app, &draw, &frame, &self);
        });
        draw.to_frame(app, &frame).unwrap();
    }
}

#[derive(Debug)]
pub struct Drop {
    x: f32,
    y: f32,
    z: f32,
    len: f32,
    yspeed: f32,
}

impl Drop {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            len: map_range(z, 0., 20., 10., 20.),
            yspeed: map_range(z, 0., 20., 1., 20.),
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn len(&self) -> f32 {
        self.len
    }

    pub fn yspeed(&self) -> f32 {
        self.yspeed
    }

    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    pub fn set_yspeed(&mut self, yspeed: f32) {
        self.yspeed = yspeed;
    }

    pub fn update(&mut self, app: &App, _update: Update, window_id: window::Id) {
        let (_, height) = window(app, window_id).inner_size_points();
        self.set_y(self.y() - self.yspeed());
        let grav = map_range(self.z(), 0., 20., 0., 0.2);
        self.set_yspeed(self.yspeed() + grav);

        if self.y() < -(height/2.) {
            self.set_y(thread_rng().gen_range(50..500) as f32);
            self.set_yspeed(map_range(self.z(), 0., 20., 4., 10.));
        }
    }

    pub fn draw(&self, _app: &App, draw: &Draw, _frame: &Frame, _space: &Space) {
        let thickness = map_range(self.z(), 0., 20., 1., 3.);

        draw.line()
            .stroke_weight(thickness)
            .color(rgb8(138, 43, 226))
            .start(vec2(self.x(), self.y()))
            .end(vec2(self.x(), self.y() + self.len()));
    }
}
