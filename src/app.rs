pub mod start {
    use crate::{components::main_page, event::app_events, shared::player_state::PlayerState};
    use ratatui::{prelude::CrosstermBackend, Terminal};
    use std::io;

    pub fn init() {
        // pre checks
        check_for_requirements();

        // main functionalities
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal =
            Terminal::new(backend).unwrap_or_else(|_| panic!("unable to create a terminal ui"));
        let mut player_state = PlayerState::default();
        loop {
            terminal
                .draw(|f| main_page::application::draw(f, &mut player_state))
                .unwrap_or_else(|_| panic!("unable to load ratatui in a terminal ui"));
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
    pub fn check_for_requirements() {
        // check for env
        let url = std::env::var("BASE_URL").unwrap_or_else(|_| panic!("unable to load env!"));

        // check for internet connectivity
        let client = reqwest::blocking::Client::new();
        let response = client.get(url).send();
        response.unwrap_or_else(|_| panic!("please have a valid internet connection"));
    }
}
