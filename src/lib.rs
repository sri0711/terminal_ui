pub mod app;

pub mod components {
    pub mod main_page;
    pub mod notification_page;
    pub mod player_screen;
    pub mod search_popup;
    pub mod song_list;
}

pub mod event {
    pub mod app_events;
}

pub mod helpers {
    pub mod audio_decode;
    pub mod core_audio_module;
}

pub mod shared {
    pub mod audio_state;
    pub mod player_state;
}

pub mod types {
    pub mod song_model;
}
