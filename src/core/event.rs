use crossterm::event::{self, Event as CEvent};
use std::time::Duration;

#[derive(Debug)]
pub enum AppEvent {
    Key(crossterm::event::KeyEvent),
    Tick,
}

pub async fn next_event() -> Result<AppEvent, std::io::Error> {
    if event::poll(Duration::from_millis(50))? {
        if let CEvent::Key(key) = event::read()? {
            return Ok(AppEvent::Key(key));
        }
    }

    Ok(AppEvent::Tick)
}
