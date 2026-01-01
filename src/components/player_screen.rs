use crate::{components::notification::notification, shared::player_state::PlayerState};

pub trait PlayerScreen {
    fn load_selected_song(player_state: &mut PlayerState);
    fn fetch_song_buffer(player_state: &mut PlayerState);
}

impl PlayerScreen for PlayerState {
    fn load_selected_song(player_state: &mut PlayerState) {
        if player_state
            .audio_state
            .selected_song_object
            .as_ref()
            .unwrap()
            .title
            .clone()
            == player_state.audio_state.selected_song
        {
            return;
        }
        let base_url = player_state
            .audio_state
            .selected_song_object
            .as_ref()
            .unwrap()
            .more_info
            .as_ref()
            .unwrap()
            .encrypted_media_url
            .as_ref()
            .unwrap()
            .to_string();
        let selected_song_url = match player_state
            .audio_state
            .clone()
            .selected_song_object
            .unwrap()
            .more_info
            .unwrap()
            .is_320
        {
            Some(ref s) if s == "true" => base_url.replace("96", "320"),
            Some(ref s) if s == "false" => base_url.replace("96", "128"),
            Some(_) => todo!(),
            None => todo!(),
        };

        player_state.audio_state.audio_url = selected_song_url;
        player_state.audio_state.selected_song = Some(
            html_escape::decode_html_entities(
                player_state
                    .audio_state
                    .selected_song_object
                    .as_ref()
                    .unwrap()
                    .title
                    .as_ref()
                    .unwrap()
                    .as_str(),
            )
            .to_string(),
        );
    }

    fn fetch_song_buffer(player_state: &mut PlayerState) {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let url = player_state.audio_state.audio_url.clone();

            let client = reqwest::Client::new();

            let response = client.get(url).send().await.unwrap();
            let bytes = response.bytes().await.unwrap_or_else(|_| {
                let message = "Failed to fetch audio buffer".to_string();
                notification::Options {
                    level: notification::Level::Error,
                    message: message,
                    ..notification::Options::default()
                };
                vec![].into()
            });
            player_state.audio_state.audio_buffer = Some(bytes.to_vec());
        });

        notification::Options {
            level: notification::Level::Success,
            message: format!(
                "Now Playing: {}",
                player_state.audio_state.selected_song.clone().unwrap()
            ),
            ..notification::Options::default()
        }
        .show();
    }
}

pub mod init {
    use crate::shared::player_state::PlayerState;
    use ratatui::{
        layout::Rect,
        widgets::{Block, Borders},
        Frame,
    };

    pub fn component(frame: &mut Frame, area: Rect, player_state: &mut PlayerState) {
        if player_state.audio_state.selected_song_object.is_some() {
            <PlayerState as super::PlayerScreen>::load_selected_song(player_state);
        }

        if player_state.audio_state.selected_song.is_some() {}
        let text = Block::default().borders(Borders::all());

        if player_state.audio_state.selected_song.is_some() {
            let title = player_state
                .audio_state
                .selected_song
                .as_ref()
                .unwrap()
                .to_string();
            let titled_text = text.title(format!(" Now Playing: {} ", title));
            frame.render_widget(titled_text, area);
            return;
        }

        frame.render_widget(text, area);
    }
}
