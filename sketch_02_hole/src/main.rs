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
  Model {}
}

fn update(_app: &App, m: &mut Model, _update: Update) {

}

fn view(app: &App, m: &Model, frame: Frame) {
  // Begin drawing
  let draw = &(app.draw());
  draw.background().color(WHITE);
  draw
      .rect()
      .xy(vec2(0.0, 0.0))
      .w_h(120.0, 180.0)
      .hsl(random_f32() * 20.0, 0.8, 0.8);

  // let file_path = captured_frame_path(app, &frame);
  // app.main_window().capture_frame(file_path);

  // Write the result of our drawing to the window's frame.
  draw.to_frame(app, &frame).unwrap();
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
  app.project_path()
      .expect("failed to locate `project_path`")
      .join("./cap/")
      .join(app.exe_name().unwrap())
      .join(format!("{:04}", frame.nth()))
      .with_extension("png")
}