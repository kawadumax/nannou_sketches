use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 640;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
}

fn model(app: &App) -> Model {
  app.set_loop_mode(LoopMode::loop_once());
  app.new_window().size(WINDOW_WIDTH, WINDOW_HEIGHT).view(view).build().unwrap();
  let (_w, h) = app.window_rect().w_h();
  Model {
  }
}

fn update(_app: &App, m: &mut Model, _update: Update) {
}

fn view(app: &App, m: &Model, frame: Frame) {
  // Begin drawing
  let draw = &(app.draw());
  draw.background().color(BLACK);
  let ww = WINDOW_WIDTH as f32;
  let wh = WINDOW_HEIGHT as f32;
  cantor(draw, -ww/2.0, 0.0, ww, 1.0);


  let file_path = captured_frame_path(app, &frame);
  app.main_window().capture_frame(file_path);

  // Write the result of our drawing to the window's frame.
  draw.to_frame(app, &frame).unwrap();
}

fn cantor(draw: &Draw, x: f32, y: f32, length: f32, weight: f32) {
  if length < 0.1 { return; }
  let h = random_f32() * 250.0;
  draw.ellipse()
      .no_fill()
      .stroke(WHITE)
      .stroke_weight(1.0)
      .hsl(h, 0.8, 0.7)
      .x_y(x + length/2.0, y)
      .w_h(length, length);
  draw.line()
      .stroke_weight(weight)
      .hsl(h, 0.8, 0.7)
      .start(vec2(x, y - weight/2.0))
      .end(vec2(x + length, y - weight/2.0));
  draw.line()
      .stroke_weight(weight)
      .hsl(h, 0.8, 0.7)
      .start(vec2(x + weight/2.0, y - length/2.0))
      .end(vec2(x + weight/2.0,  y + length/2.0));
  draw.line()
      .stroke_weight(weight)
      .hsl(h, 0.8, 0.7)
      .start(vec2(x + length - weight/2.0, y - length/2.0))
      .end(vec2(x + length - weight/2.0,  y + length/2.0));
  let lower_y = y - length/4.0;
  cantor(draw, x, lower_y, length/3.0, weight);
  cantor(draw, x + length/3.0 * 2.0, lower_y, length/3.0, weight);
  let upper_y = y + length/4.0;
  cantor(draw, x, upper_y, length/3.0, weight);
  cantor(draw, x + length/3.0 * 2.0, upper_y, length/3.0, weight);

}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("./cap/")
        .join(app.exe_name().unwrap())
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}