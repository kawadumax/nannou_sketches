
use nannou::prelude::*;
// use nannou::math;
use crate::WINDOW_WIDTH;

// A simple particle type
pub struct Particle {
  position: Point2,
  velocity: Vector2,
  acceleration: Vector2,
  life_span: f32,
  hue: f32,
  age: f32,
  scale: f32
}

impl Particle {
  pub fn new(l: Point2) -> Self {
    let acceleration = vec2(0.0, 0.02);
    let velocity = vec2(Self::random_f32_width(2.0), random_f32() - 1.0);
    let position = l + vec2(Self::random_f32_width(WINDOW_WIDTH as f32), 0.0);
    let life_span = 400.0;
    let hue = random_f32() * 20.0;
    let age = 0.0;
    let scale = 1.0;

    Particle {
      acceleration,
      velocity,
      position,
      life_span,
      hue,
      age,
      scale
    }
  }

  // Method to update position
  pub fn update(&mut self) {
    self.velocity += self.acceleration;
    self.position -= self.velocity / 2.0;
    self.life_span -= 1.0;
    self.age += 0.1;
    self.scale = 1.0 + pow(self.age, 2) / 200.0;
  }

  // Method to display
  pub fn display(&self, draw: &Draw) {
    let size = 28.0;
    let scale = self.scale;
    draw
      .scale(scale)
      .ellipse()
      .xy(self.position/scale)
      .w_h(size, size)
      .hsl(self.hue, 0.8, 0.8);

    self.draw_wing(draw);
    
    let eye_size = vec2(2.0 , 3.0);
    draw
      .scale(scale)
      .ellipse()
      .x((self.position.x + 4.0 * scale)/scale)
      .y((self.position.y + 4.0 * scale)/scale)
      .wh(eye_size)
      .hsl(self.hue, 0.8, 0.1);
    draw
      .scale(scale)
      .ellipse()
      .x((self.position.x - 4.0 * scale)/scale)
      .y((self.position.y + 4.0 * scale)/scale)
      .wh(eye_size)
      .hsl(self.hue, 0.8, 0.1);
    
    // くちばし
    // let p1 = pt2(self.position.x / scale , (self.position.x + 2.0 * scale)/scale);
    // let p2 = pt2((self.position.x + 2.0 * scale)/scale, (self.position.x - 2.0 * scale)/scale);
    // let p3 = pt2((self.position.x - 2.0 * scale)/scale, (self.position.x - 2.0 * scale)/scale);
    // let p4 = pt2(self.position.x / scale , (self.position.x + 2.0 * scale)/scale);
    let p1 = pt2(0.0, 2.0);
    let p2 = pt2(-2.0, 0.0);
    let p4 = pt2(2.0, 0.0);
    let p3 = pt2(0.0 , -2.0);
    draw
      .scale(scale)
      .quad()
      .xy(self.position/scale)
      .hsl(self.hue, 0.8, 0.1)
      .points(p1, p2, p3,p4);
    
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
    // let scale = 1.0 + self.age/10.0;
    let radius = 50.0 * self.scale;
    let weight = 3.0 * self.scale;
    let angle = PI / 6.0 * self.age.cos();

    draw
      // .scale(scale)
      .line()
      .stroke_weight(weight)
      .start(self.position)
      .end(self.position + vec2(radius * angle.cos(), radius * angle.sin()))
      .hsl(self.hue, 0.8, 0.8);
    draw
      // .scale(scale)
      .line()
      .stroke_weight(weight)
      .start(self.position)
      .end(self.position + vec2(-radius * angle.cos(), radius * angle.sin()))
      .hsl(self.hue, 0.8, 0.8);
  }
}

