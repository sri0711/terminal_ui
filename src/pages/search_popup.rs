pub mod init {
    use crate::state::player_state::PlayerState;
    use ratatui::{
        layout::{Constraint, Direction, Layout, Rect},
        style::Stylize,
        widgets::Block,
        Frame,
    };

    pub trait SearchProperties {
        fn toggle(player_state: &mut PlayerState);
    }

    impl SearchProperties for PlayerState {
        fn toggle(player_state: &mut PlayerState) {
            player_state.show_search = !player_state.show_search;
            return;
        }
    }

    pub fn component(frame: &mut Frame, inner_layout: Rect, _player_state: &mut PlayerState) {
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

        frame.render_widget(textbox, search_box[1]);
    }
}
