use crate::state::player_state::PlayerState;

pub trait SongListProperties {
    fn fetch_songs(player_state: &mut PlayerState);
}

impl SongListProperties for PlayerState {
    fn fetch_songs(player_state: &mut PlayerState) {
        player_state.show_search = !player_state.show_search;
        player_state.trigger_search = !player_state.trigger_search;
    }
}

pub mod init {
    pub fn component() {}
}
