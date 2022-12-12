use colored::Colorize;
use std::sync::Arc;
use std::{thread};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use rand::Rng;
use rdev::{Event, EventType, listen};
use rdev::Key::{KeyZ};
use crate::clicker_class::Clicker;

mod clicker_class;


fn main() {
    let cps: u64 = 10;
    let r1: u64 = 1000 / cps - 1;
    let r2: u64 = 1000 / cps + 1;
    let dl1: u64 = 0;
    let dl2: u64 = 5;

    println!("{}", "CClicker 0.1".yellow());
    println!("{} {} {}", "Press".yellow(), "Z".bright_blue(), "to start/stop clicking".yellow());
    println!("{} {} {} {} {}", "Range is set from".yellow(), ((r1 - dl2) as f64 / 10.0).to_string().bright_blue(), "CPS to".yellow(), ((r2 - dl1) as f64 / 10.0).to_string().bright_blue(), "CPS".yellow());
    let sc = Arc::new(AtomicBool::new(false));
    let scr = sc.clone();

    let mut clicker = Clicker::new();

    thread::spawn(move || {
        let callback = move |event: Event| {
            if event.event_type == EventType::KeyPress(KeyZ) {
                scr.store(!scr.load(Ordering::Relaxed), Ordering::Relaxed);
            }
        };

        let _ = listen(callback);
    });

    let mut rng = rand::thread_rng();

    loop {
        if sc.load(Ordering::Relaxed) {
            let d1: u64 = rng.gen_range(dl1..dl2);

            let d2: u64 = rng.gen_range(r1..r2);

            clicker.click(d1);
            thread::sleep(Duration::from_millis(d2));
        }
    }
}




