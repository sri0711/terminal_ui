use crate::{shared::audio_state::AudioState, types::song_model::SearchSongMain};
use ratatui::widgets::ListState;

#[derive(Clone, Default)]
pub struct PlayerState {
    pub show_search: bool,
    pub input: String,
    pub cursor: usize,
    pub trigger_search: bool,
    pub song_list: Option<SearchSongMain>,
    pub song_name_list: Option<Vec<String>>,
    pub audio_state: AudioState,
    pub highlight_state: ListState,
}
