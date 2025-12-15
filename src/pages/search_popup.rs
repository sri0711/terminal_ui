pub mod init {
    use ratatui::{layout::Rect, widgets::Block, Frame};

    use crate::state::player_state::PlayerState;

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
        let textbox = Block::bordered().title("search");

        frame.render_widget(textbox, inner_layout);
    }
}
