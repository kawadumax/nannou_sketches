use nannou::prelude::*;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 640;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    app.new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, m: &mut Model, _update: Update) {}

fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = &app.draw();
    draw.background().color(WHITE);
    let a = 11.0;
    let b = 7.0;
    let ratio = 1.0 / PI * 2.3;
    let half_window_height = WINDOW_HEIGHT as f32 / 2.0;
    let half_window_width = WINDOW_WIDTH as f32 / 2.0;
    let mut position = vec2(-half_window_width, -half_window_height);

    let mut width = WINDOW_WIDTH as f32;
    let mut itr_count = 0;

    // タイル作成

    while width > 0.1 {
        itr_count += 1;
        if itr_count % 2 == 1 {
            while position.x + width * ratio < half_window_width + 0.1 {
                draw_tile(draw, position.x, position.y, width * ratio, width);
                position.x += width * ratio;
            }
            width = half_window_width - position.x;
        } else {
            while position.y + width / ratio < half_window_width + 0.1 {
                draw_tile(draw, position.x, position.y, width, width / ratio);
                position.y += width / ratio;
            }
            width = half_window_width as f32 - position.y;
        }
    }

    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn draw_tile(draw: &Draw, x: f32, y: f32, w: f32, h: f32) {
    let translate_vec = vec3(w / 2.0, h / 2.0, 0.0);
    draw.translate(translate_vec)
        .rect()
        .xy(vec2(x, y))
        .w_h(w, h)
        .hsl(random_f32() * 250.0, 0.8, 0.7);
    draw.translate(translate_vec)
        .ellipse()
        .x_y(x, y)
        .w_h(w, w)
        .hsl(random_f32() * 250.0, 0.8, 0.8);
    let pupil_size = w * (random_f32() * 0.4 + 0.3);
        draw.translate(translate_vec)
        .ellipse()
        .x_y(x, y)
        .w_h(pupil_size, pupil_size)
        .hsl(random_f32() * 250.0, 0.8, 0.7);
    // draw
    //     .arrow()
    //     .start(vec2(x, y))
    //     .end(vec2(random_f32() * 420.0,random_f32() * 520.0))
    //     // .w_h(w,h)
    //     .hsla(0.0, 0.0, 0.0, 50.0);
    // let arc = nannou::geom::path()
    //     .arc(vec2(x, y), vec2(100.0, 100.0), PI / 2.0, 0.0)
    //     .build();

    // draw.translate(translate_vec)
    //     .path()
    //     .stroke()
    //     .color(BLACK)
    //     .events(arc.iter());
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("./cap/")
        .join(app.exe_name().unwrap())
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}
