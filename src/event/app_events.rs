pub mod handler {
    use crate::{
        components::search_popup::init::SearchProperties, shared::player_state::PlayerState,
    };
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
    use std::time::Duration;

    pub fn read_event() -> Option<Event> {
        if event::poll(Duration::from_millis(0)).ok()? {
            event::read().ok()
        } else {
            None
        }
    }

    pub fn exit(event: &Event) -> bool {
        matches!(
            event,
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            })
        )
    }

    pub fn application(player_state: &mut PlayerState, event: &Event) {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            if player_state.show_search {
                match (code, modifiers) {
                    (KeyCode::Enter | KeyCode::Char('\r') | KeyCode::Char('\n'), _) => {
                        <PlayerState as SearchProperties>::begin_search(player_state);
                    }

                    // for search input handle
                    (KeyCode::Char(c), &KeyModifiers::NONE) => {
                        player_state.input.insert(player_state.cursor, c.to_owned());
                        player_state.cursor = player_state.input.chars().count();
                    }

                    // Backspace
                    (KeyCode::Backspace, _) if player_state.cursor > 0 => {
                        player_state.cursor -= 1;
                        player_state.input.remove(player_state.cursor);
                    }

                    // Move cursor left
                    (KeyCode::Left, _) if player_state.cursor > 0 => {
                        player_state.cursor -= 1;
                    }

                    // Move cursor right
                    (KeyCode::Right, _) if player_state.cursor < player_state.input.len() => {
                        player_state.cursor += 1;
                    }
                    _ => {}
                }
            }
            if let (KeyCode::Char('k'), &KeyModifiers::CONTROL) = (code, modifiers) {
                <PlayerState as SearchProperties>::toggle(player_state);
            }
        }
    }
}
