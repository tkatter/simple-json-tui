use ratatui::text::Text;

use crate::app::ValueType;

#[derive(Debug)]
pub struct SelectionScreen {
    pub options: Vec<&'static str>,
}

impl Default for SelectionScreen {
    fn default() -> Self {
        Self {
            options: vec!["String", "Number", "Bool", "Object", "Array"],
        }
    }
}
