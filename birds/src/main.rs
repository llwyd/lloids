use nannou::prelude::*;

mod bird;
pub use crate::bird::Bird;

struct Model {
    bird:Vec<Bird>,
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
        bird: Vec::new(),
    };

    model.bird.push(Bird::new(pt2(0.0, 0.0)));
    model.bird.push(Bird::new(pt2(0.0, 50.0)));
    model.bird.push(Bird::new(pt2(0.0, -50.0)));
    
    model
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent)
{
}

fn event(_app: &App, _model: &mut Model, _event: Event) { }

fn is_bird_nearby(bird: &Bird, other_bird: &Bird) -> bool{
    let bird_radius = bird.radius();

    let dx_2:f32 = (other_bird.position().x - bird.position().x).pow(2);
    let dy_2:f32 = (other_bird.position().y - bird.position().y).pow(2);
    let other_bird_radius = (dx_2 + dy_2).sqrt();
    println!("r:{},{}",bird_radius, other_bird_radius);
    (other_bird_radius <= bird_radius)
}

fn update(app: &App, model: &mut Model, update: Update) { 
    let win = app.window_rect();

    let num_bird = model.bird.len();
    for i in 0..num_bird{

        /* Collect nearby birds */
        let mut nearby:Vec<Bird> = Vec::new();
        for j in 0..num_bird{
            if(i != j)
            {
                if is_bird_nearby(&model.bird[i], &model.bird[j])
                {
                    println!("{},{}", i, j);
                    println!("Bird is within influence range\n");
                    nearby.push(model.bird[j]);
                }
            }
        }
        /* Handle Separation */
        /* Handle Alignment */
        /* Handle Cohesion */
    }


    for bird in &mut model.bird{
        bird.update(&win);
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();
    let draw = app.draw();

    for bird in &model.bird{
        //bird.draw_region(&draw);
    }
    model.bird[1].draw_region(&draw);

    for bird in &model.bird{
        bird.draw(&draw);
    }

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

