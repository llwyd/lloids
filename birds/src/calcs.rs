pub use crate::bird::Bird;
use nannou::prelude::*;

pub fn is_bird_nearby(bird: &Bird, other_bird: &Bird, bird_radius: f32) -> bool{
    let dx_2:f32 = (other_bird.position().x - bird.position().x).pow(2);
    let dy_2:f32 = (other_bird.position().y - bird.position().y).pow(2);
    let other_bird_radius = (dx_2 + dy_2).sqrt();
    other_bird_radius <= bird_radius
}

pub fn separation(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{

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
    
    let separation_angle = deg_to_rad(90.0) - angle;

    

    println!("Separation:{:?} Angle:{},{}", average, rad_to_deg(angle), rad_to_deg(separation_angle));

    separation_angle
}

pub fn alignment(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{

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

pub fn cohesion(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{
    
    /* Calculate angles */
    let num_bird = other_birds.len();

    let mut average = pt2(0.0, 0.0);

    for i in 0..num_bird{
        average.x += other_birds[i].position().x;
        average.y += other_birds[i].position().y;
    }

    average.x /= num_bird as f32;
    average.y /= num_bird as f32;

    let mut angle = (bird.position().y - average.y).atan2(bird.position().x - average.x);
    if angle < 0.0{
        angle = angle + ( 2.0 * std::f32::consts::PI );
    }

    println!("Cohesion:{:?} Angle:{}", average, rad_to_deg(angle));

    angle
}

#[cfg(test)]
mod tests {
    use super::*;    
    
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
        assert_eq!(angle, deg_to_rad(90.0));
    }
    
    #[test]
    fn separation_angle_x_neg(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(-1.0, 0.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = separation(&mut bird, &bird_vec);
        assert_eq!(angle, deg_to_rad(-90.0));
    }
    
    #[test]
    fn separation_angle_y_pos(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(0.0, 1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = separation(&mut bird, &bird_vec);
        assert_eq!(angle, deg_to_rad(0.0));
    }
    
    #[test]
    fn separation_angle_y_neg(){
        let mut bird_vec:Vec<Bird> = Vec::new();
        
        let mut bird = Bird::new(pt2(0.0, -1.0), deg_to_rad(0.0));
        bird_vec.push(Bird::new(pt2(0.0, 0.0), deg_to_rad(0.0))); 

        let angle = separation(&mut bird, &bird_vec);
        assert_eq!(angle, -3.1415925);
    }
}

