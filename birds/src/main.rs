use nannou::prelude::*;
use nannou::geom::Range;

mod bird;
mod calcs;
pub use crate::bird::Bird;
    
const SCREEN_W_F32:f32 = 1024.0;
const SCREEN_H_F32:f32 = 768.0;

const SCREEN_W_2:f32 = SCREEN_W_F32 / 2.0;
const SCREEN_H_2:f32 = SCREEN_H_F32 / 2.0;

const SCREEN_W_U32:u32 = SCREEN_W_F32 as u32;
const SCREEN_H_U32:u32 = SCREEN_H_F32 as u32;

const SCREEN_TURN_OFFSET:f32 = 100.0;

const NUM_BIRDS:u32 = 1;

struct Model {
    bird:Vec<Bird>,
    show_radii:bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(SCREEN_W_U32, SCREEN_H_U32)
        .min_size(SCREEN_W_U32, SCREEN_H_U32)
        .max_size(SCREEN_W_U32, SCREEN_H_U32)
        //.decorations(false)
        .resizable(false)
        .event(window_event)
        .build()
        .unwrap();
    
    let mut model = Model {
        bird: Vec::new(),
        show_radii: false,
    };

    for _i in 0..NUM_BIRDS{
        let x = random_range(-SCREEN_W_2, SCREEN_W_2); 
        let y = random_range(-SCREEN_H_2, SCREEN_H_2); 
        let angle = random_range(0.0, 359.0);

        //model.bird.push(Bird::new(pt2(x, y), deg_to_rad(0.0))); 
        model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(315.0))); 
    }

    model
}

fn window_event(_app: &App, _model: &mut Model, _event: WindowEvent)
{
}

fn event(_app: &App, _model: &mut Model, _event: Event) { }






fn update(app: &App, model: &mut Model, _update: Update) { 
    let win = app.window_rect();

    let inner = Rect{
        x: Range{start: -SCREEN_W_2 + SCREEN_TURN_OFFSET, end: SCREEN_W_2 - SCREEN_TURN_OFFSET},
        y: Range{start: -SCREEN_H_2 + SCREEN_TURN_OFFSET, end: SCREEN_H_2 - SCREEN_TURN_OFFSET},
    };


    let num_bird = model.bird.len();
    for i in 0..num_bird{

        /* Collect nearby birds */
        let mut nearby:Vec<Bird> = Vec::new();
        let mut nearby_sep:Vec<Bird> = Vec::new();

        for j in 0..num_bird{
            if i != j
            {
                let sep_radius = model.bird[i].separation_radius();
                let radius = model.bird[i].radius();
                if calcs::is_bird_nearby(&model.bird[i], &model.bird[j], sep_radius)
                {
                    nearby_sep.push(model.bird[j]);
                }
                
                if calcs::is_bird_nearby(&model.bird[i], &model.bird[j], radius)
                {
                    nearby.push(model.bird[j]);
                }
            }
        }
        /* Handle Separation */
        if nearby_sep.len() > 0{
            let sep_angle = calcs::separation(&mut model.bird[i], &nearby_sep);
            model.bird[i].set_separation(-sep_angle); 
        }
        
        /* Handle Alignment */
        if nearby.len() > 0 {

            let align_angle = calcs::alignment(&mut model.bird[i], &nearby);
            model.bird[i].set_alignment(align_angle); 

            /* Handle Cohesion */
            let coh_angle = calcs::separation(&mut model.bird[i], &nearby);
            model.bird[i].set_cohesion(coh_angle); 
        }
        else
        {
            model.bird[i].set_alignment(0.0); 
        }

        model.bird[i].update(&win, &inner);
    }

}

fn view(app: &App, model: &Model, frame: Frame){
    //let win = app.window_rect();
    let draw = app.draw();

    if model.show_radii{
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(SCREEN_W_F32 - (SCREEN_TURN_OFFSET * 2.0), SCREEN_H_F32 - (SCREEN_TURN_OFFSET * 2.0))
            .color(DARKGREY);

        for bird in &model.bird{
            bird.draw_region(&draw);
        }
        
        for bird in &model.bird{
            bird.draw_sep_region(&draw);
        }
    }

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

