pub use crate::bird::Bird;
use nannou::prelude::*;

fn wrap_angle(angle: f32) -> f32{
    let ref_angle = angle % (2.0 * std::f32::consts::PI);
    let mut wrapped_angle = ref_angle;
    
    if ref_angle < 0.0{
        wrapped_angle = ref_angle + ( 2.0 * std::f32::consts::PI );
    }
    else if ref_angle >= ( 2.0 * std::f32::consts::PI ){
        wrapped_angle = ref_angle - ( 2.0 * std::f32::consts::PI ); 
    }
    
    assert!(wrapped_angle >= 0.0);
    assert!(wrapped_angle < deg_to_rad(360.0));
    wrapped_angle
}

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

pub fn separation(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{

    let average = average_position( other_birds );
    let angle = (bird.position().y - average.y).atan2(bird.position().x - average.x);
    
    assert!(angle >= -180.0);
    assert!(angle <= 180.0);

    angle
}

pub fn alignment(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{

    /* Calculate angles */
    let num_bird = other_birds.len();

    let mut average_sin = 0.0;
    let mut average_cos = 0.0;

    for i in 0..num_bird{
        average_sin += other_birds[i].angle().sin();
        average_cos += other_birds[i].angle().cos();
    }
    
    average_sin /= num_bird as f32;
    average_cos /= num_bird as f32;
   
    /* Circular mean */
    let average = average_sin.atan2(average_cos);

    
    let delta = bird.angle() - wrap_angle(average);
    
    assert!(delta != std::f32::INFINITY);
    assert!(delta != std::f32::NEG_INFINITY);
    
    assert!(delta >= -180.0);
    assert!(delta <= 180.0);

    delta
}


pub fn cohesion(bird: &mut Bird, other_birds: &Vec <Bird>)->f32
{
    let average = average_position( other_birds );
    let angle = (average.y - bird.position().y).atan2(average.x - bird.position().x);
    
    assert!(angle >= -180.0);
    assert!(angle <= 180.0);

    angle
}


#[cfg(test)]
mod tests {
    use super::*;    
    const FLOAT_PRECISION:f32 = 0.00001;
   
    fn compare_floats(x:f32, y:f32, precision:f32)->bool{

        let delta = (x - y).abs();
        delta <= precision
    }

    #[test]
    fn inside_circle(){
        let bird_0 = Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0)); 
        let bird_1 = Bird::new(pt2(0.0, 10.0), deg_to_rad(0.0)); 

        let radius = 15.0;
        let inside = is_bird_nearby(&bird_0, &bird_1,radius);
        assert!(inside);
    }
    
    #[test]
    fn inside_circle_exactly(){
        let bird_0 = Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0)); 
        let bird_1 = Bird::new(pt2(0.0, 15.0), deg_to_rad(0.0)); 

        let radius = 15.0;
        let inside = is_bird_nearby(&bird_0, &bird_1,radius);
        assert!(inside);
    }
    
    #[test]
    fn not_inside_circle(){
        let bird_0 = Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0)); 
        let bird_1 = Bird::new(pt2(0.0, 15.000001), deg_to_rad(0.0)); 

        let radius = 15.0;
        let inside = is_bird_nearby(&bird_0, &bird_1,radius);
        assert!(!inside);
    }
    
    #[test]
    fn separation_angle_x_pos(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, 0.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = separation(&mut bird, &bird_vec);
        assert_eq!(angle, deg_to_rad(0.0));
    }
    
    #[test]
    fn separation_angle_x_neg(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, 0.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = separation(&mut bird, &bird_vec);
        assert_eq!(angle, deg_to_rad(180.0));
    }
    
    #[test]
    fn separation_angle_y_pos(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(0.0, 1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(90.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn separation_angle_y_neg(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(0.0, -1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(-90.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn separation_angle_ne(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, 1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(45.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn separation_angle_nw(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, 1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(135.0), FLOAT_PRECISION));
    }
    #[test]
    fn separation_angle_se(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, -1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(-45.0), FLOAT_PRECISION));
    }

    #[test]
    fn separation_angle_sw(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, -1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = separation(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(-135.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn cohesion_angle_x_pos(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, 0.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert_eq!(angle, std::f32::consts::PI);
    }
    
    #[test]
    fn cohesion_angle_x_neg(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, 0.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert_eq!(angle, deg_to_rad(0.0));
    }
    
    #[test]
    fn cohesion_angle_y_pos(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(0.0, 1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(-90.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn cohesion_angle_y_neg(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(0.0, -1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(90.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn cohesion_angle_ne(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, 1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(-135.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn cohesion_angle_nw(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, 1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(-45.0), FLOAT_PRECISION));
    }
    #[test]
    fn cohesion_angle_se(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(1.0, -1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(135.0), FLOAT_PRECISION));
    }

    #[test]
    fn cohesion_angle_sw(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, -1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = cohesion(&mut bird, &bird_vec);
        assert!(compare_floats(angle, deg_to_rad(45.0), FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_single_pos_x(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(1.0, 0.0), deg_to_rad(0.0))); 

        let average_position = average_position(&bird_vec);
        assert!(compare_floats(average_position.x, 1.0, FLOAT_PRECISION));
        assert!(compare_floats(average_position.y, 0.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_single_neg_x(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(-1.0, 0.0), deg_to_rad(0.0))); 

        let average_position = average_position(&bird_vec);
        assert!(compare_floats(average_position.x, -1.0, FLOAT_PRECISION));
        assert!(compare_floats(average_position.y, 0.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_single_pos_y(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(0.0, 1.0), deg_to_rad(0.0))); 

        let average_position = average_position(&bird_vec);
        assert!(compare_floats(average_position.x, 0.0, FLOAT_PRECISION));
        assert!(compare_floats(average_position.y, 1.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_single_neg_y(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(0.0, -1.0), deg_to_rad(0.0))); 

        let average_position = average_position(&bird_vec);
        assert!(compare_floats(average_position.x, 0.0, FLOAT_PRECISION));
        assert!(compare_floats(average_position.y, -1.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_2_pos(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(0.0))); 
        bird_vec.push(Bird::new(pt2(1.0, 2.0), deg_to_rad(0.0))); 

        let average_position = average_position(&bird_vec);
        assert!(compare_floats(average_position.x, 1.0, FLOAT_PRECISION));
        assert!(compare_floats(average_position.y, 2.0, FLOAT_PRECISION));
    }
    
    #[test]
    fn calc_average_4_corners(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        bird_vec.push(Bird::new(pt2(1.0, 1.0), deg_to_rad(0.0))); 
        bird_vec.push(Bird::new(pt2(1.0, -1.0), deg_to_rad(0.0))); 
        bird_vec.push(Bird::new(pt2(-1.0, 1.0), deg_to_rad(0.0))); 
        bird_vec.push(Bird::new(pt2(-1.0, -1.0), deg_to_rad(0.0))); 

        let average_position = average_position(&bird_vec);
        assert!(compare_floats(average_position.x, 0.0, FLOAT_PRECISION));
        assert!(compare_floats(average_position.y, 0.0, FLOAT_PRECISION));
    }
}

