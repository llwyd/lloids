use nannou::prelude::*;

pub use crate::settings::Settings;
pub use crate::bird::BirdConfig;

#[derive(Copy, Clone, PartialEq, Debug)]
enum SettingSelection{
    Nowt = 0,
    Separation = 1,
    Cohesion = 2,
    Alignment = 3,
    SpeedMin = 4,
    SpeedMax = 5,
}

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
    PausePress,
    PauseRelease,
    CyclePress,
    CycleRelease,
    IncrementPress,
    IncrementRelease,
    DecrementPress,
    DecrementRelease,
}

#[derive(Copy, Clone)]
pub struct KeyPress{
    input:BirdInput,
    changed:bool,
    selection:SettingSelection,
}

impl KeyPress
{
    pub fn new() -> KeyPress{
        KeyPress{
            input: BirdInput::Nowt,
            changed: false,
            selection: SettingSelection::Nowt,
        }
    }

    fn increment_selection(&mut self, settings: &mut Settings)
    {
        if settings.show_debug {
            match self.selection{
                SettingSelection::Nowt => self.selection = SettingSelection::Separation,
                SettingSelection::Separation => self.selection = SettingSelection::Cohesion,
                SettingSelection::Cohesion => self.selection = SettingSelection::Alignment,
                SettingSelection::Alignment => self.selection = SettingSelection::SpeedMin,
                SettingSelection::SpeedMin => self.selection = SettingSelection::SpeedMax,
                SettingSelection::SpeedMax => self.selection = SettingSelection::Nowt,
            }
        }

    }

    pub fn changed(&self) -> bool{
        self.changed
    }

    pub fn separation_selected(&self) -> bool
    {
        self.selection == SettingSelection::Separation
    }
    
    pub fn cohesion_selected(&self) -> bool
    {
        self.selection == SettingSelection::Cohesion
    }
    
    pub fn alignment_selected(&self) -> bool
    {
        self.selection == SettingSelection::Alignment
    }
    
    pub fn speedmin_selected(&self) -> bool
    {
        self.selection == SettingSelection::SpeedMin
    }
    
    pub fn speedmax_selected(&self) -> bool
    {
        self.selection == SettingSelection::SpeedMax
    }

    pub fn handle_increment(&self, settings: &Settings, config:&mut BirdConfig)
    {
        if settings.show_debug {
            match self.selection{
                SettingSelection::Separation => config.separation.inc_delta(),
                SettingSelection::Cohesion => config.cohesion.inc_delta(),
                SettingSelection::Alignment => config.alignment_gain.increment(),
                SettingSelection::SpeedMin => config.speed.inc_min(),
                SettingSelection::SpeedMax => config.speed.inc_max(),
                _ => {},
            }
        }
    }
    
    pub fn handle_decrement(&self, settings: &Settings, config:&mut BirdConfig)
    {
        if settings.show_debug {
            match self.selection{
                SettingSelection::Separation => config.separation.dec_delta(),
                SettingSelection::Cohesion => config.cohesion.dec_delta(),
                SettingSelection::Alignment => config.alignment_gain.decrement(),
                SettingSelection::SpeedMin => config.speed.dec_min(),
                SettingSelection::SpeedMax => config.speed.dec_max(),
                _ => {},
            }
        }
    }

    pub fn update_settings(&mut self, settings: &mut Settings, config:&mut BirdConfig)
    {
        match self.input{
            BirdInput::DebugPress => settings.show_debug ^= true,
            BirdInput::TrailPress => settings.show_trails ^= true,
            BirdInput::TurnboxPress => settings.show_turnbox ^= true,
            BirdInput::RadiiPress => settings.show_radii ^= true,
            BirdInput::PausePress => settings.pause ^= true,
            BirdInput::CyclePress => self.increment_selection(settings),
            BirdInput::IncrementPress => self.handle_increment(settings, config),
            BirdInput::DecrementPress => self.handle_decrement(settings, config),
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
            Key::P => self.input = BirdInput::PausePress,
            Key::Tab => self.input = BirdInput::CyclePress,
            Key::Up => self.input = BirdInput::IncrementPress,
            Key::Down => self.input = BirdInput::DecrementPress,
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
            Key::P => self.input = BirdInput::PauseRelease,
            Key::Tab => self.input = BirdInput::CycleRelease,
            Key::Up => self.input = BirdInput::IncrementRelease,
            Key::Down => self.input = BirdInput::DecrementRelease,
            _ => self.input = BirdInput::Nowt,
        }

        self.changed = previous_input != self.input;
    }
}
