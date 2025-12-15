pub mod start {
    use crate::{event::app_events, pages::main_page, state::player_state::PlayerState};
    use ratatui::{prelude::CrosstermBackend, Terminal};
    use std::io;

    pub fn init() {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend).unwrap();
        let mut player_state = PlayerState::default();
        loop {
            terminal
                .draw(|f| main_page::application::draw(f, &mut player_state))
                .expect("unable to create a terminal");
            if let Some(event) = app_events::handler::read_event() {
                if app_events::handler::exit(&event) {
                    break;
                }
                // main application events
                app_events::handler::application(&mut player_state, &event);
            }
        }
        ratatui::restore();
    }
}
