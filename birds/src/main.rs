use nannou::prelude::*;
use nannou::geom::Range;

mod bird;
pub use crate::bird::Bird;
    
const SCREEN_W_F32:f32 = 1024.0;
const SCREEN_H_F32:f32 = 768.0;

const SCREEN_W_2:f32 = SCREEN_W_F32 / 2.0;
const SCREEN_H_2:f32 = SCREEN_H_F32 / 2.0;

const SCREEN_W_U32:u32 = SCREEN_W_F32 as u32;
const SCREEN_H_U32:u32 = SCREEN_H_F32 as u32;

const SCREEN_TURN_OFFSET:f32 = 120.0;

struct Model {
    bird:Vec<Bird>,
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
    };

    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    model.bird.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 
    
    model.bird.push(Bird::new(pt2(20.0, 20.0), deg_to_rad(0.0)));
    model.bird.push(Bird::new(pt2(0.0, -20.0), deg_to_rad(0.0)));
    model.bird.push(Bird::new(pt2(0.0, 35.0), deg_to_rad(135.0)));
    model.bird.push(Bird::new(pt2(20.0, 35.0), deg_to_rad(180.0)));
    model.bird.push(Bird::new(pt2(20.0, -35.0), deg_to_rad(225.0)));
    model.bird.push(Bird::new(pt2(180.0, -180.0), deg_to_rad(180.0)));
   
    model
}

fn window_event(_app: &App, _model: &mut Model, _event: WindowEvent)
{
}

fn event(_app: &App, _model: &mut Model, _event: Event) { }

fn is_bird_nearby(bird: &Bird, other_bird: &Bird) -> bool{
    let bird_radius = bird.radius();

    let dx_2:f32 = (other_bird.position().x - bird.position().x).pow(2);
    let dy_2:f32 = (other_bird.position().y - bird.position().y).pow(2);
    let other_bird_radius = (dx_2 + dy_2).sqrt();
    other_bird_radius <= bird_radius
}

fn is_bird_really_nearby(bird: &Bird, other_bird: &Bird) -> bool{
    let bird_radius = bird.separation_radius();

    let dx_2:f32 = (other_bird.position().x - bird.position().x).pow(2);
    let dy_2:f32 = (other_bird.position().y - bird.position().y).pow(2);
    let other_bird_radius = (dx_2 + dy_2).sqrt();
    other_bird_radius <= bird_radius
}

fn separation(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{

    /* Calculate angles */
    let num_bird = other_birds.len();

    let mut average = pt2(0.0, 0.0);

    for i in 0..num_bird{
        average.x += other_birds[i].position().x;
        average.y += other_birds[i].position().y;
    }

    average.x /= num_bird as f32;
    average.y /= num_bird as f32;

//    let angle = average.y.atan2(average.x) - bird.position().y.atan2(bird.position().x);
    let mut angle = (average.y - bird.position().y).atan2(average.x - bird.position().x);

    if angle < 0.0{
        angle = angle + ( 2.0 * std::f32::consts::PI );
    }
    

    println!("Separation:{:?} Angle:{}", average, rad_to_deg(angle));

    angle -= std::f32::consts::PI;
    if angle < 0.0{
        angle = angle + ( 2.0 * std::f32::consts::PI );
    }
    angle
}

fn alignment(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{

    /* Calculate angles */
    let num_bird = other_birds.len();

    let mut average = 0.0;

    for i in 0..num_bird{
        average += other_birds[i].angle();
    }
    
    average /= num_bird as f32;
    let delta = bird.angle() - average;
    
    
    println!("Align: {:?}, Delta{:?}", average, delta);
    assert!(delta != std::f32::INFINITY);
    assert!(delta != std::f32::NEG_INFINITY);

    delta
}

fn cohesion(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{
    
    /* Calculate angles */
    let num_bird = other_birds.len();

    let mut average = pt2(0.0, 0.0);

    for i in 0..num_bird{
        average.x += other_birds[i].position().x;
        average.y += other_birds[i].position().y;
    }

    average.x /= num_bird as f32;
    average.y /= num_bird as f32;

    //let mut angle = (average.y - bird.position().y).atan2(average.x - bird.position().x);
    let mut angle = (bird.position().y - average.y).atan2(bird.position().x - average.x);
    if angle < 0.0{
        angle = angle + ( 2.0 * std::f32::consts::PI );
    }

    println!("Cohesion:{:?} Angle:{}", average, rad_to_deg(angle));

    angle
}

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
                if is_bird_really_nearby(&model.bird[i], &model.bird[j])
                {
                    nearby_sep.push(model.bird[j]);
                }
                
                if is_bird_nearby(&model.bird[i], &model.bird[j])
                {
                    nearby.push(model.bird[j]);
                }
            }
        }
        /* Handle Separation */
        if nearby_sep.len() > 0{
            let sep_angle = separation(&mut model.bird[i], &nearby_sep);
            model.bird[i].set_separation(sep_angle); 
        }
        
        /* Handle Alignment */
        if nearby.len() > 0 {

            let align_angle = alignment(&mut model.bird[i], &nearby);
            model.bird[i].set_alignment(align_angle); 

            /* Handle Cohesion */
            let coh_angle = cohesion(&mut model.bird[i], &nearby);
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

