use nannou::prelude::*;
// use std::time::*;

pub struct Clock {
    // time:Dulation
}

impl Clock {
    pub fn new() -> Self {
        Clock {}
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, draw: &Draw) {
        const clock_size: Vector2<f32> = Vector2::new(200.0, 200.0);
        draw.ellipse()
            .no_fill()
            .stroke_weight(5.0)
            .x(100.0)
            .y(100.0)
            .wh(clock_size)
            .color(BLACK);
    }
}
