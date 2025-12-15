pub mod handler {
    use crossterm::event::{self, read, Event, KeyCode, KeyEvent, KeyModifiers};
    use std::{io::Result, time::Duration};

    use crate::{pages::search_popup::init::SearchProperties, state::player_state::PlayerState};

    pub fn exit() -> Result<bool> {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = read()?
            {
                match (code, modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                        return Ok(true);
                    }
                    _ => {}
                }
            }
        }
        Ok(false)
    }

    pub fn application(player_state: &mut PlayerState) {
        if !player_state.show_search {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = read().unwrap()
            {
                match (code, modifiers) {
                    (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                        <PlayerState as SearchProperties>::toggle(player_state);
                    }
                    _ => {}
                }
            }
        } else {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = read().unwrap()
            {
                match (code, modifiers) {
                    (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                        <PlayerState as SearchProperties>::toggle(player_state);
                    }
                    _ => {}
                }
            }
        }
    }
}
