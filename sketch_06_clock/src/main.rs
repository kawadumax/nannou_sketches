use nannou::prelude::*;

mod clock;
use clock::*;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 640;

fn main() {
  nannou::app(model).update(update).run();
}

struct Model {
  clock: Clock,
}

fn model(app: &App) -> Model {
  app.new_window().size(WINDOW_WIDTH, WINDOW_HEIGHT).view(view).build().unwrap();
  // let (_w, h) = app.window_rect().w_h();
  let clock = Clock::new();
  Model {
    clock
  }
}

fn update(_app: &App, m: &mut Model, _update: Update) {
  // if _app.elapsed_frames() % 5 == 0 { m.ps.add_particle();};

  m.clock.update();
}

fn view(app: &App, m: &Model, frame: Frame) {
  // Begin drawing
  let draw = &(app.draw());
  draw.background().color(WHITE);

  m.clock.draw(draw);

// gifに書き出したいときにコメントイン
//   let file_path = captured_frame_path(app, &frame);
//   app.main_window().capture_frame(file_path);

// Write the result of our drawing to the window's frame.
  draw.to_frame(app, &frame).unwrap();
}

// gifに書き出したいときにコメントイン
// fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
//     app.project_path()
//         .expect("failed to locate `project_path`")
//         .join("./cap/")
//         .join(app.exe_name().unwrap())
//         .join(format!("{:04}", frame.nth()))
//         .with_extension("png")
// }