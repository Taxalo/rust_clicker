use std::{thread};
use std::time::Duration;

use rdev::Button::Left;
use rdev::{ EventType, simulate};

pub struct Clicker {}

impl Clicker {

    pub fn new() -> Clicker {
        Clicker {}
    }

    pub fn click(&mut self, drop_delay: u64) {
        Self::send(&EventType::ButtonPress(Left));
        thread::sleep(Duration::from_millis(drop_delay));
        Self::send(&EventType::ButtonRelease(Left));
    }

    fn send(event_type: &EventType) {
        let _ = simulate(event_type);
    }
}