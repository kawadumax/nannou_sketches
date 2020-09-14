
use nannou::prelude::*;
use crate::WINDOW_WIDTH;

// A simple particle type
pub struct Particle {
  position: Point2,
  velocity: Vector2,
  acceleration: Vector2,
  life_span: f32,
  hue: f32
}

impl Particle {
  pub fn new(l: Point2) -> Self {
    let acceleration = vec2(0.0, 0.02);
    let velocity = vec2(Self::random_f32_width(2.0), random_f32() - 1.0);
    let position = l + vec2(Self::random_f32_width(WINDOW_WIDTH as f32), 0.0);
    let life_span = 500.0;
    let hue = random_f32() * 20.0;
    Particle {
      acceleration,
      velocity,
      position,
      life_span,
      hue
    }
  }

  // Method to update position
  pub fn update(&mut self) {
    self.velocity += self.acceleration;
    self.position -= self.velocity / 2.0;
    self.life_span -= 1.0;
  }

  // Method to display
  pub fn display(&self, draw: &Draw) {
    let size = 28.0;
    draw
      .ellipse()
      .xy(self.position)
      .w_h(size, size)
      .hsla(self.hue, 0.8, 0.8, self.life_span / 255.0);

    self.draw_wing(draw);
    
  }

  // Is the poarticel still useful?
  pub fn is_dead(&self) -> bool {
    self.life_span < 0.0 
  }

  // private methods
  fn random_f32_width(w:f32) -> f32 {
    random_f32() * w - w / 2.0
  }

  fn draw_wing(&self, draw: &Draw){
    let radius = 50.0;
    let angle = 30.0; 
    draw
      .line()
      .stroke_weight(3.0)
      .start(self.position)
      .end(self.position + vec2(radius * sin(angle), 20.0))
      .hsla(self.hue, 0.8, 0.8, self.life_span / 255.0);
    draw
      .line()
      .stroke_weight(3.0)
      .start(self.position)
      .end(self.position + vec2(-40.0, 20.0))
      .hsla(self.hue, 0.8, 0.8, self.life_span / 255.0);
  }
}
