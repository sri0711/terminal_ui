use crate::{
    components::notification_page::notification::{self, Options},
    shared::player_state::PlayerState,
    types::song_model::SearchSongMain,
};

pub trait SongListProperties {
    fn fetch_songs(player_state: &mut PlayerState);
    fn update_song_list(player_state: &mut PlayerState);
    fn song_name_list(player_state: &mut PlayerState);
}

impl SongListProperties for PlayerState {
    fn fetch_songs(player_state: &mut PlayerState) {
        if player_state.input.len() >= 3 {
            Self::update_song_list(player_state);
            player_state.trigger_search = !player_state.trigger_search;
        } else {
            Options {
                message: String::from("Please enter at least 3 characters to search."),
                level: notification::Level::Warning,
                ..Options::default()
            }
            .show();
            player_state.trigger_search = false;
            player_state.show_search = true;
        }
    }

    fn update_song_list(player_state: &mut PlayerState) {
        let rt = tokio::runtime::Runtime::new()
            .unwrap_or_else(|_| panic!("failed to spawn the blocking thread!"));

        let url = std::env::var("BASE_URL").unwrap();

        rt.block_on(async {
            let client = reqwest::Client::new();
            let resp = client
                .get(format!("{}{}", url, player_state.input))
                .send()
                .await
                .expect("Failed to fetch songs");

            if resp.status().is_success() {
                let songs: SearchSongMain =
                    resp.json().await.expect("Failed to parse songs response");
                player_state.song_list = Some(songs);
            } else {
                notification::Options {
                    message: format!("Failed to fetch songs: HTTP {}", resp.status().as_u16()),
                    level: notification::Level::Error,
                    ..notification::Options::default()
                }
                .show();
            }
        });
    }
    fn song_name_list(player_state: &mut PlayerState) {
        if let Some(song_list) = &player_state.song_list {
            if let Some(item) = &song_list.results {
                let names: Vec<String> = item
                    .iter()
                    .map(|song_data| {
                        let song_name;
                        if let Some(song_details) = song_data.more_info.clone() {
                            song_name = format!(
                                "{}-{}",
                                html_escape::decode_html_entities(
                                    &song_data.title.clone().unwrap()
                                ),
                                html_escape::decode_html_entities(
                                    &song_details.album.clone().unwrap()
                                )
                            );
                        } else {
                            song_name =
                                html_escape::decode_html_entities(&song_data.title.clone().unwrap())
                                    .to_string()
                        }
                        song_name
                    })
                    .collect();
                player_state.song_name_list = Some(names);
            }
        }
    }
}

pub mod init {
    use crate::{components::song_list::SongListProperties, shared::player_state::PlayerState};
    use ratatui::{
        layout::Rect,
        style::{Modifier, Style},
        widgets::{Block, Borders, List, ListItem},
        Frame,
    };

    pub fn component(frame: &mut Frame, area: Rect, player_state: &mut PlayerState) {
        if player_state.trigger_search {
            player_state.song_list = None;
            <PlayerState as SongListProperties>::fetch_songs(player_state);
        }
        if player_state.song_list.is_some() {
            <PlayerState as SongListProperties>::song_name_list(player_state);
        }
        let song_component = Block::default().borders(Borders::all()).title(" Songs ");
        let name_items = match player_state.song_name_list.as_ref() {
            Some(names) if !names.is_empty() => names
                .iter()
                .enumerate()
                .map(|(i, name)| ListItem::new(format!("{}. {}", i, name)))
                .collect(),
            _ => vec![ListItem::new("No songs found")],
        };
        let name_element = List::new(name_items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .block(song_component);

        frame.render_stateful_widget(name_element, area, &mut player_state.highlight_state);
    }
}
