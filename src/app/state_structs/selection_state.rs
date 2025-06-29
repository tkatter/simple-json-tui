use crate::ratatui::widgets::ListState;

#[derive(Debug)]
pub struct SelectionScreen {
    pub options: Vec<&'static str>,
    pub state: ListState,
}

impl Default for SelectionScreen {
    fn default() -> Self {
        Self {
            options: vec!["String", "Number", "Bool", "Object", "Array"],
            state: ListState::default(),
        }
    }
}
