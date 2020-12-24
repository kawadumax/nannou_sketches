use nannou::prelude::*;

// mod particle;
// // use particle::*;

// mod particle_system;
// use particle_system::*;

// const WINDOW_WIDTH: u32 = 640;
// const WINDOW_HEIGHT: u32 = 640;

fn main() {
//   nannou::app(model).update(update).run();
  nannou::sketch(view).run();
}

// struct Model {
//   ps: ParticleSystem,
// }

// fn model(app: &App) -> Model {
//   app.new_window().size(WINDOW_WIDTH, WINDOW_HEIGHT).view(view).build().unwrap();
//   let (_w, h) = app.window_rect().w_h();
//   let ps = ParticleSystem::new(pt2(0.0, (h as f32 / 2.0) - 50.0));
//   Model {
//     ps
//   }
// }

// fn update(_app: &App, m: &mut Model, _update: Update) {
//   if _app.elapsed_frames() % 5 == 0 { m.ps.add_particle();};

//   m.ps.update();
// }

fn view(app: &App, frame: Frame) {
  // Begin drawing
  let draw = &(app.draw());
  draw.background().color(WHITE);

//   m.ps.draw(draw);

//   let file_path = captured_frame_path(app, &frame);
//   app.main_window().capture_frame(file_path);

// Write the result of our drawing to the window's frame.
  draw.to_frame(app, &frame).unwrap();
}

// fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
//     app.project_path()
//         .expect("failed to locate `project_path`")
//         .join("./cap/")
//         .join(app.exe_name().unwrap())
//         .join(format!("{:04}", frame.nth()))
//         .with_extension("png")
// }