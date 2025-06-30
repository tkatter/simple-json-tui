use crate::ratatui::widgets::ListState;

#[derive(Default)]
pub struct QuittingScreen {
    pub list_size: usize,
    pub state: ListState,
}
