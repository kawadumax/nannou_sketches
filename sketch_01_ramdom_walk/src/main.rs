use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
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
    // frame.clear(BLACK);
    _model.w.display(_app, &frame);
}