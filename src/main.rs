use chrono::{DateTime, Utc};
use rdev::{grab, Event, EventType, Key};
use screenshots::Screen;
use std::env;
use std::fs;
use std::io::Result;

const TARGET_DIR: &str = "screens";

fn main() {
    if let Ok(dir) = create_screens_dir() {
        grab_event(dir);
    }
}

fn create_screens_dir() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    let screens_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    let mut path = env::current_dir()?;
    path.push(&screens_dir);
    if let Err(error) = fs::create_dir_all(path) {
        println!("Error: {:?}", error);
        return Err(error);
    }
    Ok(screens_dir)
}

fn grab_event(dir: String) {
    if let Err(error) = grab(move |e| -> Option<Event> {
        match e.event_type {
            EventType::KeyPress(Key::PrintScreen) => {
                make_screen(&dir);
                None
            }
            _ => Some(e),
        }
    }) {
        println!("Error: {:?}", error);
    }
}

fn make_screen(screen_dir: &String) {
    println!("{screen_dir}");
    let screens = Screen::all().unwrap();
    for screen in screens {
        let image = screen.capture().unwrap();
        let now: DateTime<Utc> = Utc::now();
        image
            .save(format!(
                "{}/{}.png",
                screen_dir,
                now.format("%d-%m-%Y_%H_%M_%S")
            ))
            .unwrap();
    }
}
