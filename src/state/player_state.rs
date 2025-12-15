#[derive(Clone, Default)]
pub struct PlayerState {
    pub show_search: bool,
    pub input: String,
    pub cursor: usize,
}
