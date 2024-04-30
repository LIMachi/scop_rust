use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub enum InputStatus {
    #[default]
    Released,
    Pressed,
    JustPressed,
    JustReleased
}

impl InputStatus {
    pub fn new(pressed: bool) -> Self {
        if pressed {
            Self::JustPressed
        } else {
            Self::JustReleased
        }
    }
    
    pub fn pressed(&self) -> bool {
        match self {
            InputStatus::Pressed | InputStatus::JustPressed => true,
            _ => false
        }
    }

    pub fn released(&self) -> bool {
        match self {
            InputStatus::Released | InputStatus::JustReleased => true,
            _ => false
        }
    }
    
    pub fn just_released(&self) -> bool {
        self == &InputStatus::JustReleased
    }

    pub fn just_pressed(&self) -> bool {
        self == &InputStatus::JustPressed
    }
    
    fn tick(&mut self) {
        match self {
            InputStatus::JustPressed => { 
                *self = InputStatus::Pressed;
            }
            InputStatus::JustReleased => {
                *self = InputStatus::Released;
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct InputHandler<T: Eq + PartialEq + Hash + Debug + Copy + Clone> {
    mapper: HashMap<VirtualKeyCode, T>,
    status: HashMap<T, InputStatus>
}

impl <T: Eq + PartialEq + Hash + Debug + Copy + Clone> Default for InputHandler<T> {
    fn default() -> Self {
        Self {
            mapper: HashMap::new(),
            status: HashMap::new()
        }
    }
}

impl<T: Eq + PartialEq + Hash + Debug + Copy + Clone> InputHandler<T> {
    pub fn map(&mut self, input: T, key: VirtualKeyCode) {
        self.mapper.insert(key, input);
        if !self.status.contains_key(&input) {
            self.status.insert(input, InputStatus::Released);
        }
    }
    
    pub fn tick(&mut self) {
        for (_, status) in self.status.iter_mut() {
            status.tick();
        }
    }
    
    pub fn key_event(&mut self, input: KeyboardInput) {
        if let Some(status) = input.virtual_keycode
            .and_then(|k| self.mapper.get(&k))
            .and_then(|i| self.status.get_mut(i)) {
            let press = input.state == ElementState::Pressed;
            if status.pressed() != press {
                *status = InputStatus::new(press);
            }
        }
    }
    
    pub fn status(&self, input: T) -> InputStatus {
        self.status.get(&input).copied().unwrap_or(InputStatus::Released)
    }
    
    pub fn pressed<'s>(&'s self) -> impl IntoIterator<Item = T> + 's {
        self.status.iter().filter_map(|(i, s)| {
            if s.pressed() {
                Some(*i)
            } else {
                None
            }
        })
    }
}