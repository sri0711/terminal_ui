use crate::state::player_state::PlayerState;

impl PlayerState {}

pub mod application {
    use ratatui::{
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::Stylize,
        symbols::border,
        widgets::{Block},
        Frame,
    };

    use crate::state::player_state::PlayerState;

    pub fn draw(frame: &mut Frame, _: &mut PlayerState) {
        let outline = Block::bordered()
            .title("> Sri Juke Box <")
            .title_alignment(Alignment::Center)
            .border_set(border::DOUBLE)
            .green();
        frame.render_widget(outline.clone(), frame.area());

        let inner = outline.inner(frame.area());
        main_page(frame, inner);
    }

    pub fn main_page(frame: &mut Frame, inner_layout: Rect) {
        let main_screen_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Fill(3)])
            .split(inner_layout);

        let text = Block::default();

        frame.render_widget(text.clone(), main_screen_layout[0]);
        frame.render_widget(text, main_screen_layout[1]);
    }

    fn left_container (frame: &mut Frame ,outer_layer:Block)  {
        let left_sections = Layout::default().direction(Direction::Vertical).constraints([Constraint::Fill(1), Constraint::Fill(2)]).split(outer_layer);
    }
}
