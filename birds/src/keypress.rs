use nannou::prelude::*;

pub use crate::settings::Settings;

#[derive(Copy, Clone, PartialEq, Debug)]
enum BirdInput{
    Nowt,
    DebugPress,
    DebugRelease,
    TrailPress,
    TrailRelease,
}

#[derive(Copy, Clone)]
pub struct KeyPress{
    input:BirdInput,
    changed:bool
}

impl KeyPress
{
    pub fn new() -> KeyPress{
        KeyPress{
            input: BirdInput::Nowt,
            changed: false,
        }
    }

    pub fn changed(&self) -> bool{
        self.changed
    }

    pub fn update_settings(&self, settings: &mut Settings)
    {
        match self.input{
            BirdInput::DebugPress => settings.show_debug ^= true,
            BirdInput::TrailPress => settings.show_trails ^= true,
            _ => {},
        }
    }

    pub fn reset_latch(&mut self){
        self.changed = false;
    }

    pub fn handle_press(&mut self, key: Key){
        let previous_input = self.input;
        match key{
            Key::D => self.input = BirdInput::DebugPress,
            Key::T => self.input = BirdInput::TrailPress,
            _ => self.input = BirdInput::Nowt,
        }

        self.changed = previous_input != self.input;
        if self.changed{
            println!("Handle press");
        }
    }
    
    pub fn handle_release(&mut self, key: Key){
        let previous_input = self.input;
        match key{
            Key::D => self.input = BirdInput::DebugRelease,
            Key::T => self.input = BirdInput::TrailRelease,
            _ => self.input = BirdInput::Nowt,
        }

        self.changed = previous_input != self.input;
        if self.changed{
            println!("Handle release");
        }
    }
}
