use nannou::prelude::*;

mod bird;
pub use crate::bird::Bird;

struct Model {
    bird:Bird,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(640,480)
        .min_size(640,480)
        .max_size(640,480)
        //.decorations(false)
        .resizable(false)
        .event(window_event)
        .build()
        .unwrap();
    
    let mut model = Model {
        bird: Bird::new(),
    };

    model
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent)
{
}

fn event(_app: &App, _model: &mut Model, _event: Event) { }

fn update(app: &App, model: &mut Model, update: Update) { 
    let win = app.window_rect();
    model.bird.update(&win);
}

fn view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();
    let draw = app.draw();

    model.bird.draw(&draw);

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

