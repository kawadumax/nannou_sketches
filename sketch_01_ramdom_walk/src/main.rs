use nannou::prelude::*;

fn main() {

    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}

struct Walker {
    x: i32,
    y: i32
}

impl Walker {

    fn step(&mut self) {
        
        let step_x = random_range(-2, 3);
        let step_y = random_range(-2, 3);
        self.x += step_x;
        self.y += step_y;
    }

    fn display(&self, _app: &App, frame:&Frame) {
        println!("{},{}",self.x, self.y);
        let draw = _app.draw();
        let off_black = rgba(1.0,1.0,1.0,0.2);
        draw.quad().x_y(self.x as f32, self.y as f32).w_h(1.0, 1.0).color(off_black);
        draw.to_frame(_app, &frame).unwrap();
    }
}

struct Model {
    w:Walker,
}

fn model(_app: &App) -> Model {
    std::fs::remove_dir_all(_app.project_path()
        .expect("failed to locate `project_path`")
        .join("./cap/")
        .join(_app.exe_name().unwrap())).unwrap();
    _app.new_window().size(100, 100).build().unwrap();
    Model {
        w: Walker {
            x:0,
            y:0,
        }
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.w.step();
}

fn view(_app: &App, _model: &Model, frame: Frame){
    if _app.elapsed_frames() == 1 { frame.clear(BLACK) };
    _model.w.display(_app, &frame);

    // Capture the frame!
    //
    // NOTE: You can speed this up with `capture_frame_threaded`, however be aware that if the
    // image writing threads can't keep up you may quickly begin to run out of RAM!
    let file_path = captured_frame_path(_app, &frame);
    _app.main_window().capture_frame(file_path);
}

fn captured_frame_path(_app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    _app.project_path()
        .expect("failed to locate `project_path`")
        .join("./cap/")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(_app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:04}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}