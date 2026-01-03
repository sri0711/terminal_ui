use crate::types::song_model::SearchSongMain;

#[derive(Clone, Default)]
pub struct PlayerState {
    pub show_search: bool,
    pub input: String,
    pub cursor: usize,
    pub trigger_search: bool,
    pub song_list: Option<SearchSongMain>,
    pub song_name_list: Option<Vec<String>>,
}
