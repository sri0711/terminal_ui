use crate::state::player_state::PlayerState;

impl PlayerState {}

pub mod application {
    use ratatui::{Frame, style::Stylize, widgets::Paragraph};

    use crate::state::player_state::PlayerState;

    pub fn draw(frame: &mut Frame, _: &mut PlayerState) {
        let text = Paragraph::new("sample").green().bold();

        frame.render_widget(text, frame.area());
    }
}
