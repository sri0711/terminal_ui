use crate::types::song_model::SearchSongResult;

#[derive(Default, Clone)]
pub struct AudioState {
    pub selected_song_object: Option<SearchSongResult>,
    pub selected_song: Option<String>,
    pub audio_url: String,
    pub audio_buffer: Option<Vec<u8>>,
    pub is_playing: bool,
}
