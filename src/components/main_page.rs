pub mod application {

    // imports for this mod
    use crate::{
        components::{search_popup, song_list},
        shared::player_state::PlayerState,
    };
    use ratatui::{
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::Stylize,
        symbols::border,
        widgets::{Block, Borders},
        Frame,
    };

    pub fn draw(frame: &mut Frame, player_state: &mut PlayerState) {
        let outline = Block::bordered()
            .title("> Sri Juke Box <")
            .title_alignment(Alignment::Center)
            .border_set(border::DOUBLE)
            .green();
        frame.render_widget(outline.clone(), frame.area());

        let inner = outline.inner(frame.area());
        main_page(frame, inner, player_state);
    }

    pub fn main_page(frame: &mut Frame, inner_layout: Rect, player_state: &mut PlayerState) {
        if !player_state.show_search {
            let main_screen_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Fill(1), Constraint::Fill(3)])
                .split(inner_layout);

            let text = Block::default().borders(Borders::all());

            left_container(frame, main_screen_layout[0], player_state);
            frame.render_widget(text, main_screen_layout[1]);
        } else {
            search_popup::init::component(frame, inner_layout, player_state);
        }
    }

    fn left_container(frame: &mut Frame, outer_layer: Rect, player_state: &mut PlayerState) {
        let left_sections = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Fill(2)])
            .split(outer_layer);

        let text_btm = Block::default()
            .borders(Borders::all())
            .title(" B.L.E List ");

        song_list::init::component(frame, left_sections[0], player_state);
        frame.render_widget(text_btm, left_sections[1]);
    }
}
