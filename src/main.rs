use model::{Drop, Space};
use nannou::prelude::*;
use rand::{thread_rng, Rng};
use utils::window;

pub mod model;
pub mod utils;

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Space {
    let window_id = app
        .new_window()
        .view(view)
        .title("Purple Rain")
        .build()
        .unwrap();

    let (width, _) = window(app, window_id).inner_size_points();

    let drops = (1..500)
        .map(|_| {
            Drop::new(
                thread_rng().gen_range(-width..width),
                thread_rng().gen_range(50..500) as f32,
                thread_rng().gen_range(0..20) as f32,
            )
        })
        .collect();

    Space::new(window_id, drops)
}

fn update(app: &App, space: &mut Space, update: Update) {
    space.update(app, update)
}

fn view(app: &App, space: &Space, frame: Frame) {
    space.draw(app, frame);
}
