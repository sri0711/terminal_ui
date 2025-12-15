pub mod init {
    use crate::state::player_state::PlayerState;
    use ratatui::{
        layout::{Constraint, Direction, Layout, Rect},
        style::Stylize,
        widgets::{Block, Paragraph},
        Frame,
    };

    pub trait SearchProperties {
        fn toggle(player_state: &mut PlayerState);
        fn begin_search(player_state: &mut PlayerState);
    }

    impl SearchProperties for PlayerState {
        fn toggle(player_state: &mut PlayerState) {
            player_state.show_search = !player_state.show_search;
        }

        fn begin_search(player_state: &mut PlayerState) {
            player_state.trigger_search = !player_state.trigger_search
        }
    }

    pub fn component(frame: &mut Frame, inner_layout: Rect, player_state: &mut PlayerState) {
        let search_box_vertical = Layout::default()
            .constraints([
                Constraint::Percentage(42),
                Constraint::Percentage(16),
                Constraint::Percentage(42),
            ])
            .direction(Direction::Vertical)
            .split(inner_layout);

        let search_box = Layout::default()
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(70),
                Constraint::Percentage(15),
            ])
            .direction(Direction::Horizontal)
            .split(search_box_vertical[1]);

        let textbox = Block::bordered().title(" Search ").bold();
        let text = format!("-> {}", player_state.input);
        let text_element = Paragraph::new(text).block(textbox);
        frame.render_widget(text_element, search_box[1]);
        let search_box_position = search_box[1];
        let cursor_x = search_box_position.x + 4 + player_state.cursor as u16;
        let cursor_y = search_box_position.y + 1;
        frame.set_cursor_position((cursor_x, cursor_y));
    }
}
