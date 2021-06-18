use std::collections::HashMap;

use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct State {
    pub keys: HashMap<VirtualKeyCode, KeyCondition>,
    pub page: Page,
}

#[derive(Debug)]
pub enum KeyCondition {
    First,
    Repeat,
}

#[derive(Debug)]
pub enum Page {
    Intro,
    ModelView,
    LevelView,
}

impl State {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            page: Page::Intro,
        }
    }

    pub fn handle_window_event(&mut self, event: &Event<'static, ()>) {
        match event {
            Event::WindowEvent {
                event: window_event,
                ..
            } => match window_event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_keycode),
                            state: element_state,
                            ..
                        },
                    ..
                } => match element_state {
                    ElementState::Pressed if self.keys.contains_key(&virtual_keycode) => {
                        self.keys.insert(*virtual_keycode, KeyCondition::Repeat);
                        self.on_key_repeat(*virtual_keycode);
                    }
                    ElementState::Pressed => {
                        self.keys.insert(*virtual_keycode, KeyCondition::First);
                        self.on_key_pressed(*virtual_keycode);
                    }
                    ElementState::Released => {
                        self.keys.remove(&virtual_keycode);
                        self.on_key_released(*virtual_keycode);
                    }
                },
                WindowEvent::Focused(true) => self.on_focus(),
                WindowEvent::Focused(false) => self.on_blur(),
                _ => {}
            },
            Event::DeviceEvent {
                event: device_event,
                ..
            } => match device_event {
                DeviceEvent::MouseMotion { delta } => self.on_mouse_motion(*delta),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn on_key_pressed(&mut self, key: VirtualKeyCode) {
        println!("on_key_pressed {:?} {:?}", key, self.keys);
    }

    pub fn on_key_repeat(&mut self, key: VirtualKeyCode) {
        println!("on_key_repeat {:?} {:?}", key, self.keys);
    }

    pub fn on_key_released(&mut self, key: VirtualKeyCode) {
        println!("on_key_released {:?} {:?}", key, self.keys);
    }

    // pub fn on_key_modifiers_changed(&mut self, modifiers: ModifiersState) {
    //     // println!("on_key_modifiers_changed {:?}", modifiers);
    // }

    pub fn on_focus(&mut self) {
        // println!("on_focus");
    }

    pub fn on_blur(&mut self) {
        // println!("on_blur");
    }

    pub fn on_mouse_motion(&mut self, _delta: (f64, f64)) {
        // println!("on_mouse_motion {:?}", delta);
    }

    pub fn update(&mut self) {
        // Hmm, I will have to pass the state through arguments, or just update the GUI here.
        // self.gui.update();
    }
}
