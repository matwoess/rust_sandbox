extern crate chrono;

use std::fmt::{Display, Formatter};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local};

pub struct Note {
    pub title: String,
    pub text: String,
    changed_at: u64, // time since epoch
}

pub const DIVIDER    : &str = "------------- ";
pub const PAD_TITLE  : &str = "Title       : ";
pub const PAD_TEXT   : &str = "Text        : ";
pub const PAD_CHANGED: &str = "Last change : ";

impl Note {
    pub fn new(title: String, text: String) -> Note {
        let now = current_datetime();
        return Note {
            title,
            text,
            changed_at: now,
        };
    }

    pub fn update(&mut self, new_title: String, new_text: String) {
        let mut changed = false;
        if !new_title.is_empty() && new_title != self.title {
            self.title = new_title;
            changed = true;
            println!("> updated title.");
        }
        if !new_text.is_empty() && new_text != self.text {
            self.text = new_text;
            changed = true;
            println!("> updated title.");
        }
        if changed {
            self.changed_at = current_datetime();
            println!("> updated changed_at.");
        } else {
            println!("> no changes made.");
        }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dt = UNIX_EPOCH + Duration::from_secs(self.changed_at);
        let dt_format = DateTime::<Local>::from(dt).format("%b %d, %k:%M").to_string();
        writeln!(f, "{}", DIVIDER)?;
        writeln!(f, "{}{}", PAD_TITLE, self.title)?;
        writeln!(f, "{}{}", PAD_TEXT, self.text)?;
        writeln!(f, "{}{}", PAD_CHANGED, dt_format)?;
        writeln!(f, "{}", DIVIDER)
    }
}

fn current_datetime() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(time) => time.as_secs(),
        Err(_) => {
            println!("SystemTime::now before UNIX_EPOCH");
            0
        }
    }
}
