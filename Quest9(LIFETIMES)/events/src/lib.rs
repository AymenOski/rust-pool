use colored::*;
use std::{fmt, time::Duration};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Clone, Copy)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (c1, c2, c3) = self.color;
        let content = self.content.truecolor(c1, c2, c3);
        write!(f, "({:?}, {}, {})", self.position, self.size, content)
    }
}

impl Event<'_> {
    pub fn notify(self) -> Notification {
        match self {
            Event::Remainder(s) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: s.to_string(),
            },
            Event::Registration(duration) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!(
                    "You have {} left before the registration ends",
                    format_duration(duration)
                ),
            },
            Event::Appointment(s) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: s.to_string(),
            },
            _ => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}
fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let h = total_seconds / 3600;
    let m = (total_seconds % 3600) / 60;
    let s = total_s % 60;
    format!("{}H:{:02}M:{:02}S", h, m, seconds)
}
