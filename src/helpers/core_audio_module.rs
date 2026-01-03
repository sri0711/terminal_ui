use crate::shared::player_state::PlayerState;

pub trait AudioModule {
    fn play_audio(player_state: &mut PlayerState);
}

impl AudioModule for PlayerState {
    fn play_audio(_player_state: &mut PlayerState) {
        // Implementation for playing audio
    }
}
