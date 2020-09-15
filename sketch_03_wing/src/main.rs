// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// example 4-03: Particle System Type

use nannou::prelude::*;

mod particle;
// use particle::*;

mod particle_system;
use particle_system::*;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 640;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  ps: ParticleSystem,
}

fn model(app: &App) -> Model {
  app.new_window().size(WINDOW_WIDTH, WINDOW_HEIGHT).view(view).build().unwrap();
  let (_w, h) = app.window_rect().w_h();
  let ps = ParticleSystem::new(pt2(0.0, (h as f32 / 2.0) - 50.0));
  Model {
    ps
  }
}

fn update(_app: &App, m: &mut Model, _update: Update) {
  if _app.elapsed_frames() % 5 == 0 { m.ps.add_particle();};

  m.ps.update();
}

fn view(app: &App, m: &Model, frame: Frame) {
  // Begin drawing
  let draw = &(app.draw());
  draw.background().color(WHITE);

  m.ps.draw(draw);

  // Write the result of our drawing to the window's frame.
  draw.to_frame(app, &frame).unwrap();
}
