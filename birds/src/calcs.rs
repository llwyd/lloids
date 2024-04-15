pub use crate::bird::Bird;
use nannou::prelude::*;
use crate::angle;

pub fn is_bird_nearby(bird: &Bird, other_bird: &Bird, bird_radius: f32) -> bool{
    let dx_2:f32 = (other_bird.position().x - bird.position().x).pow(2);
    let dy_2:f32 = (other_bird.position().y - bird.position().y).pow(2);
    let other_bird_radius = (dx_2 + dy_2).sqrt();
    other_bird_radius <= bird_radius
}

fn average_position(bird: &Vec <Bird>) -> Point2{
    
    /* Calculate angles */
    let num_bird = bird.len();
    assert!(num_bird > 0);

    let mut average = pt2(0.0, 0.0);

    for i in 0..num_bird{
        average.x += bird[i].position().x;
        average.y += bird[i].position().y;
    }

    average.x /= num_bird as f32;
    average.y /= num_bird as f32;

    average
}

fn average_angle(bird: &Vec <Bird>) -> f32
{
    /* Calculate angles */
    let num_bird = bird.len();

    let mut average_sin = 0.0;
    let mut average_cos = 0.0;

    for i in 0..num_bird{
        assert!( bird[i].angle() >= 0.0);
        assert!( bird[i].angle() < 2.0 * std::f32::consts::PI);
        average_sin += bird[i].angle().sin();
        average_cos += bird[i].angle().cos();
    }
    
    average_sin /= num_bird as f32;
    average_cos /= num_bird as f32;
   
    /* Circular mean */
    let average = average_sin.atan2(average_cos);
    assert!(average >= -std::f32::consts::PI);
    assert!(average <= std::f32::consts::PI);
    
    average
}

fn angle_delta(a:f32, b:f32) -> f32
{
    angle::wrap_180(a - b)
}

pub fn separation(bird: &mut Bird, other_birds: &Vec <Bird>)->(f32, f32){

    let average = average_position( other_birds );
    let avg_angle = average_angle(other_birds);
    let angle = (bird.position().y - average.y).atan2(bird.position().x - average.x);
    
    assert!(angle >= -std::f32::consts::PI);
    assert!(angle <= std::f32::consts::PI);

    (angle, avg_angle)
}

pub fn alignment(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{ 
    /* Circular mean */
    let average = average_angle(other_birds);
    
    let delta = angle_delta(average, bird.angle());
    
    assert!(delta != std::f32::INFINITY);
    assert!(delta != std::f32::NEG_INFINITY);
    
    assert!(delta >= -std::f32::consts::PI);
    assert!(delta <= std::f32::consts::PI);

    delta
}


pub fn cohesion(bird: &mut Bird, other_birds: &Vec <Bird>)->(f32,f32)
{
    let average = average_position( other_birds );
    let avg_angle = average_angle(other_birds);
    let angle = (average.y - bird.position().y).atan2(average.x - bird.position().x);
    
    assert!(angle >= -std::f32::consts::PI);
    assert!(angle <= std::f32::consts::PI);

    (angle, avg_angle)
}


#[cfg(test)]
mod tests {
    use super::*;    
    use crate::speed::Speed;
    use crate::proximity::ProximitySettings;
    use crate::bird::BirdConfig;
    const FLOAT_PRECISION:f32 = 0.00001;
   
    fn cmp_floats(x:f32, y:f32, precision:f32)->bool{

        let delta = (x - y).abs();
        delta <= precision
    }

    fn default_bird_config() -> BirdConfig{

        let speed = 1.0;
        let rotation_angle = deg_to_rad(1.0);

        let config = BirdConfig{
            separation: ProximitySettings::new(Speed::new(speed, speed, false), rotation_angle),
            cohesion: ProximitySettings::new(Speed::new(speed, speed, false), -rotation_angle),
            alignment_gain: 0.0,
        };

        config
    }

    #[test]
    fn inside_circle(){        
        let config = default_bird_config();
        let bird_0 = Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0),config); 
        let bird_1 = Bird::new(pt2(0.0, 10.0), deg_to_rad(0.0), config); 

        let radius = 15.0;
        let inside = is_bird_nearby(&bird_0, &bird_1,radius);
        assert!(inside);
    }
    
    #[test]
    fn inside_circle_exactly(){
        let config = default_bird_config();
        let bird_0 = Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config); 
        let bird_1 = Bird::new(pt2(0.0, 15.0), deg_to_rad(0.0), config); 

        let radius = 15.0;
        let inside = is_bird_nearby(&bird_0, &bird_1,radius);
        assert!(inside);
    }
    
    #[test]
    fn not_inside_circle(){
        let config = default_bird_config();
        let bird_0 = Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config); 
        let bird_1 = Bird::new(pt2(0.0, 15.000001), deg_to_rad(0.0), config); 

        let radius = 15.0;
        let inside = is_bird_nearby(&bird_0, &bird_1,radius);
        assert!(!inside);
    }
    
    #[test]
    fn separation_angle_x_pos(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, 0.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = separation(&mut bird, &bird_vec);
        assert_eq!(angle.0, deg_to_rad(0.0));
    }
    
    #[test]
    fn separation_angle_x_neg(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, 0.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = separation(&mut bird, &bird_vec);
        assert_eq!(angle.0, deg_to_rad(180.0));
    }
    
    #[test]
    fn separation_angle_y_pos(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(0.0, 1.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0),config )); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(90.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn separation_angle_y_neg(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(0.0, -1.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(-90.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn separation_angle_ne(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, 1.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(45.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn separation_angle_nw(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, 1.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(135.0), FLOAT_PRECISION));
    }
    #[test]
    fn separation_angle_se(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, -1.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(-45.0), FLOAT_PRECISION));
    }

    #[test]
    fn separation_angle_sw(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, -1.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(-135.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn cohesion_angle_x_pos(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, 0.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert_eq!(angle.0, std::f32::consts::PI);
    }
    
    #[test]
    fn cohesion_angle_x_neg(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, 0.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert_eq!(angle.0, deg_to_rad(0.0));
    }
    
    #[test]
    fn cohesion_angle_y_pos(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(0.0, 1.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(-90.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn cohesion_angle_y_neg(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(0.0, -1.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(90.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn cohesion_angle_ne(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, 1.0), deg_to_rad(0.0),config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(-135.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn cohesion_angle_nw(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, 1.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(-45.0), FLOAT_PRECISION));
    }
    #[test]
    fn cohesion_angle_se(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, -1.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(135.0), FLOAT_PRECISION));
    }

    #[test]
    fn cohesion_angle_sw(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, -1.0), deg_to_rad(0.0), config);
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0), config)); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(cmp_floats(angle.0, deg_to_rad(45.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_single_pos_x(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(1.0, 0.0), deg_to_rad(0.0),config)); 

        let average_position = average_position(&bird_vec);
        assert!(cmp_floats(average_position.x, 1.0, FLOAT_PRECISION));
        assert!(cmp_floats(average_position.y, 0.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_single_neg_x(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(-1.0, 0.0), deg_to_rad(0.0),config)); 

        let average_position = average_position(&bird_vec);
        assert!(cmp_floats(average_position.x, -1.0, FLOAT_PRECISION));
        assert!(cmp_floats(average_position.y, 0.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_single_pos_y(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(0.0, 1.0), deg_to_rad(0.0),config)); 

        let average_position = average_position(&bird_vec);
        assert!(cmp_floats(average_position.x, 0.0, FLOAT_PRECISION));
        assert!(cmp_floats(average_position.y, 1.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_single_neg_y(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(0.0, -1.0), deg_to_rad(0.0), config)); 

        let average_position = average_position(&bird_vec);
        assert!(cmp_floats(average_position.x, 0.0, FLOAT_PRECISION));
        assert!(cmp_floats(average_position.y, -1.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_2_pos(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(0.0), config)); 
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(0.0), config)); 

        let average_position = average_position(&bird_vec);
        assert!(cmp_floats(average_position.x, 1.0, FLOAT_PRECISION));
        assert!(cmp_floats(average_position.y, 2.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_4_corners(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(1.0, 1.0), deg_to_rad(0.0), config)); 
        bird_vec.push(Bird::new(pt2(1.0, -1.0), deg_to_rad(0.0), config)); 
        bird_vec.push(Bird::new(pt2(-1.0, 1.0), deg_to_rad(0.0), config)); 
        bird_vec.push(Bird::new(pt2(-1.0, -1.0), deg_to_rad(0.0), config)); 

        let average_position = average_position(&bird_vec);
        assert!(cmp_floats(average_position.x, 0.0, FLOAT_PRECISION));
        assert!(cmp_floats(average_position.y, 0.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_angle_delta(){

        assert_eq!(0.0, angle_delta(0.0, 0.0));
        assert_eq!(-1.0, angle_delta(0.0, 1.0));
        assert_eq!(1.0, angle_delta(1.0, 0.0));
        
        assert!(cmp_floats(deg_to_rad(-5.0), angle_delta(deg_to_rad(45.0), deg_to_rad(50.0)),FLOAT_PRECISION));
        assert!(cmp_floats(deg_to_rad(5.0), angle_delta(deg_to_rad(50.0), deg_to_rad(45.0)),FLOAT_PRECISION));
    
        assert!(cmp_floats(deg_to_rad(-1.0), angle_delta(deg_to_rad(270.0), deg_to_rad(271.0)),FLOAT_PRECISION));

        /*Average angle is 10, bird is 270, so 270 + 100 wraps round to 10 */
        assert!(cmp_floats(deg_to_rad(100.0), angle_delta(deg_to_rad(10.0), deg_to_rad(270.0)),FLOAT_PRECISION));
        assert!(cmp_floats(deg_to_rad(180.0), angle_delta(deg_to_rad(-90.0), deg_to_rad(90.0)),FLOAT_PRECISION));
        assert!(cmp_floats(deg_to_rad(180.0), angle_delta(deg_to_rad(90.0), deg_to_rad(-90.0)),FLOAT_PRECISION));
        
        assert!(cmp_floats(deg_to_rad(-100.0), angle_delta(deg_to_rad(270.0), deg_to_rad(10.0)),FLOAT_PRECISION));
        assert!(cmp_floats(deg_to_rad(-179.0), angle_delta(deg_to_rad(180.0), deg_to_rad(359.0)),FLOAT_PRECISION));
        assert!(cmp_floats(deg_to_rad(179.0), angle_delta(deg_to_rad(180.0), deg_to_rad(1.0)),FLOAT_PRECISION));

    }    
    
    #[test]
    fn calc_average_angle_zeros(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(0.0), config)); 
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(0.0), config)); 
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(0.0), config)); 

        let average_angle = average_angle(&bird_vec);
        assert!(cmp_floats(average_angle, 0.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_angle_45(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(45.0),config)); 
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(45.0),config)); 
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(45.0),config)); 

        let average_angle = average_angle(&bird_vec);
        assert!(cmp_floats(average_angle, deg_to_rad(45.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_angle_90_90_270(){
        let config = default_bird_config();
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(90.0),config)); 
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(90.0),config)); 
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(270.0),config)); 

        let average_angle = average_angle(&bird_vec);
        println!("{:?}", average_angle);
        assert!(cmp_floats(average_angle, deg_to_rad(90.0), FLOAT_PRECISION));
    }
}

