use nannou::prelude::*;
use nannou::geom::Range;

mod bird;
mod calcs;
mod angle;
mod keypress;
mod settings;
mod meta;
mod speed;
mod proximity;
mod gain;

pub use crate::bird::Bird;
pub use crate::bird::BirdConfig;
pub use crate::speed::Speed;
pub use crate::gain::Gain;
pub use crate::proximity::ProximitySettings;
pub use crate::proximity::Proximity;

//pub use crate::bird::Speed;
pub use crate::keypress::KeyPress;
pub use crate::settings::Settings;
pub use crate::meta::Meta;

const SCREEN_W_F32:f32 = 1920.0;
const SCREEN_H_F32:f32 = 1080.0;

const SCREEN_W_2:f32 = SCREEN_W_F32 / 2.0;
const SCREEN_H_2:f32 = SCREEN_H_F32 / 2.0;

const SCREEN_TURN_OFFSET:f32 = 250.0;
const SCREEN_TURN_OFFSET_HARD:f32 = 80.0;


/* Bird default settings */
const NUM_BIRDS:u32 = 150;

const SPEED_GAIN:f32 = 1.4;
const DEFAULT_BIRD_SPEED_MIN:f32 = 1.0 * SPEED_GAIN;
const DEFAULT_BIRD_SPEED_MAX:f32 = 7.5 * SPEED_GAIN;

const DEFAULT_SEP_SPEED_MIN:f32 = 1.25 * SPEED_GAIN;
const DEFAULT_SEP_SPEED_MAX:f32 = 2.5 * SPEED_GAIN;

const DEFAULT_COH_SPEED_MIN:f32 = 0.5 * SPEED_GAIN;
const DEFAULT_COH_SPEED_MAX:f32 = 1.5 * SPEED_GAIN;
    
const DEFAULT_SEP_DELTA:f32 = 0.00625 * 2.4;
const DEFAULT_COH_DELTA:f32 = 0.00005625 * 3.0;
const DEFAULT_ALIGNMENT_GAIN:f32 = 0.0275;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct Model {
    bird:Vec<Bird>,
    input:KeyPress,
    settings:Settings,
    bird_config:BirdConfig,
    meta:Meta,
}

fn model(app: &App) -> Model {
    app.new_window()
        //.size(SCREEN_W_U32, SCREEN_H_U32)
        //.min_size(SCREEN_W_U32, SCREEN_H_U32)
        //.max_size(SCREEN_W_U32, SCREEN_H_U32)
        .decorations(false)
        .resizable(false)
        .fullscreen()
        .event(window_event)
        .build()
        .unwrap();
    
    let mut model = Model {
        bird: Vec::new(),
        settings: Settings{
            show_radii: false,
            show_turnbox: false,
            show_trails: false,
            show_debug: false,
            pause: false,
        },
        bird_config:BirdConfig{
            separation:ProximitySettings::new(Speed::new(DEFAULT_SEP_SPEED_MIN,DEFAULT_SEP_SPEED_MAX, true), DEFAULT_SEP_DELTA),
            cohesion:ProximitySettings::new(Speed::new(DEFAULT_COH_SPEED_MIN,DEFAULT_COH_SPEED_MAX, true), -DEFAULT_COH_DELTA),  
            alignment_gain: Gain::new(DEFAULT_ALIGNMENT_GAIN),
            speed: Speed::new(DEFAULT_BIRD_SPEED_MIN, DEFAULT_BIRD_SPEED_MAX, true),
        },
        input: KeyPress::new(),
        meta: Meta::new(),
    };

    for _i in 0..NUM_BIRDS{
        let x = random_range(-SCREEN_W_2 + SCREEN_TURN_OFFSET, SCREEN_W_2 - SCREEN_TURN_OFFSET); 
        let y = random_range(-SCREEN_H_2 + SCREEN_TURN_OFFSET, SCREEN_H_2 - SCREEN_TURN_OFFSET); 
        let angle = random_range(0.0, 359.0);

        model.bird.push(Bird::new(pt2(x, y), deg_to_rad(angle),model.bird_config)); 
    }

    model
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent)
{
    /* Handle keypress */
    match event{
        KeyPressed(key) => model.input.handle_press(key), 
        KeyReleased(key) => model.input.handle_release(key), 
        _ => {}
    }

    if model.input.changed(){
        model.input.update_settings(&mut model.settings, &mut model.bird_config);
        
        for bird in &mut model.bird{
            bird.refresh_settings(&mut model.bird_config);
        }
        model.input.reset_latch();
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) { }






fn update(app: &App, model: &mut Model, _update: Update) { 
    let win = app.window_rect();

    let inner = Rect{
        x: Range{start: win.left() + SCREEN_TURN_OFFSET, end: win.right() - SCREEN_TURN_OFFSET},
        y: Range{start: win.bottom() + SCREEN_TURN_OFFSET, end: win.top() - SCREEN_TURN_OFFSET},
    };

    
    let inner_hard = Rect{
        x: Range{start: win.left() + SCREEN_TURN_OFFSET_HARD, end: win.right() - SCREEN_TURN_OFFSET_HARD},
        y: Range{start: win.bottom() + SCREEN_TURN_OFFSET_HARD, end: win.top() - SCREEN_TURN_OFFSET_HARD},
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
            model.bird[i].set_separation(sep_angle.0, sep_angle.1); 
        }
        
        /* Handle Alignment */
        if nearby.len() > 0 {

            let align_angle = calcs::alignment(&mut model.bird[i], &nearby);
            model.bird[i].set_alignment(align_angle); 

            /* Handle Cohesion */
            let coh_angle = calcs::cohesion(&mut model.bird[i], &nearby);
            model.bird[i].set_cohesion(coh_angle.0, coh_angle.1); 
        }
        else
        {
            model.bird[i].set_alignment(0.0); 
        }

        if !model.settings.pause
        {
            model.bird[i].update(&win, &inner, &inner_hard);
        }
    }

    if !model.settings.pause
    {
        model.meta.update();
    }
}

fn draw_text(draw:&Draw, font_size:u32, xy:Point2, text:String, highlighted: bool){
    if highlighted{
        draw.text(&text)
            .font_size(font_size)
            .no_line_wrap()
            .left_justify()
            .color(RED)
            .xy(xy);
    }
    else
    {
        draw.text(&text)
            .font_size(font_size)
            .no_line_wrap()
            .left_justify()
            .color(WHITE)
            .xy(xy);
    }
}

fn draw_meta(app: &App, model: &Model, config: &BirdConfig, meta: &Meta, draw: &Draw)
{
    let win = app.window_rect();
    let mut position = pt2(win.left() + 125.0, win.top() - 20.0);
    draw_text(draw, 20, position, format!("Iterations: {}", meta.iterations()),false);
    position.y -= 20.0;
    draw_text(draw, 20, position, format!("Runtime: {}s", meta.runtime().as_secs()),false);
    position.y -= 40.0;
    
    draw_text(draw, 20, position, format!("Separation Delta: {} rads", config.separation.delta()), model.input.separation_selected());
    position.y -= 20.0;
    draw_text(draw, 20, position, format!("Cohesion Delta: {} rads", config.cohesion.delta()), model.input.cohesion_selected());
    position.y -= 20.0;
    draw_text(draw, 20, position, format!("Alignment Gain: {}", config.alignment_gain.gain()), model.input.alignment_selected());
    position.y -= 40.0; 
    
    draw_text(draw, 20, position, format!("Speed(min): {}", config.speed.min()), model.input.speedmin_selected());
    position.y -= 20.0;
    draw_text(draw, 20, position, format!("Speed(max): {}", config.speed.max()), model.input.speedmax_selected());
    position.y -= 20.0;
    

    let version    = format!("v{}", VERSION);
    draw.text(&version)
        .font_size(20)
        .no_line_wrap()
        .right_justify()
        .xy(pt2(win.right() - 115.0, win.bottom() +20.0));
    
}

fn view(app: &App, model: &Model, frame: Frame){
    //let win = app.window_rect();
    let draw = app.draw();
    let win = app.window_rect();
    if model.settings.show_turnbox
    {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h((2.0*win.right()) - (SCREEN_TURN_OFFSET_HARD * 2.0), (2.0*win.top()) - (SCREEN_TURN_OFFSET_HARD * 2.0))
            .rgba8(120, 120, 120, 16);
        
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h((2.0 * win.right()) - (SCREEN_TURN_OFFSET * 2.0), (2.0 * win.top() ) - (SCREEN_TURN_OFFSET * 2.0))
            .rgba8(90, 90, 90, 16);
    }
    
    if model.settings.show_radii{
        for bird in &model.bird{
            bird.draw_region(&draw);
        }
        
        for bird in &model.bird{
            bird.draw_sep_region(&draw);
        }
    }

    if model.settings.show_trails{
        for bird in &model.bird{
            bird.draw_trail(&draw);
        }
    }

    if model.settings.show_debug{
        draw_meta(&app, &model, &model.bird_config, &model.meta, &draw);
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

