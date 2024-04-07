use nannou::prelude::*;

pub use crate::settings::Settings;

#[derive(Copy, Clone, PartialEq, Debug)]
enum BirdInput{
    Nowt,
    DebugPress,
    DebugRelease,
    TrailPress,
    TrailRelease,
    RadiiPress,
    RadiiRelease,
    TurnboxPress,
    TurnboxRelease,
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
            BirdInput::TurnboxPress => settings.show_turnbox ^= true,
            BirdInput::RadiiPress => settings.show_radii ^= true,
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
            Key::R => self.input = BirdInput::RadiiPress,
            Key::B => self.input = BirdInput::TurnboxPress,
            _ => self.input = BirdInput::Nowt,
        }

        self.changed = previous_input != self.input;
    }
    
    pub fn handle_release(&mut self, key: Key){
        let previous_input = self.input;
        match key{
            Key::D => self.input = BirdInput::DebugRelease,
            Key::T => self.input = BirdInput::TrailRelease,
            Key::R => self.input = BirdInput::RadiiRelease,
            Key::B => self.input = BirdInput::TurnboxRelease,
            _ => self.input = BirdInput::Nowt,
        }

        self.changed = previous_input != self.input;
    }
}
