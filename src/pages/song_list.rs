use crate::state::player_state::PlayerState;

pub trait SongListProperties {
    fn fetch_songs(player_state: &mut PlayerState);
}

impl SongListProperties for PlayerState {
    fn fetch_songs(player_state: &mut PlayerState) {
        player_state.trigger_search = !player_state.trigger_search;
    }
}

pub mod init {
    use crate::{pages::song_list::SongListProperties, state::player_state::PlayerState};
    use ratatui::{
        layout::Rect,
        widgets::{Block, Borders},
        Frame,
    };

    pub fn component(frame: &mut Frame, area: Rect, player_state: &mut PlayerState) {
        if player_state.trigger_search {
            <PlayerState as SongListProperties>::fetch_songs(player_state);
        }
        let text = Block::default().borders(Borders::all()).title(" Songs.. ");

        frame.render_widget(text, area);
    }
}
