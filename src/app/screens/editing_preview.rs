use serde_json::Map;

pub struct EditingPreview {
    preview_pairs: Map<String, serde_json::Value>,
}

impl Default for EditingPreview {
    fn default() -> Self {
        EditingPreview {
            preview_pairs: Map::new(),
        }
    }
}

impl EditingPreview {
    pub fn push(self: &mut Self, key: String, value: serde_json::Value) {
        self.preview_pairs.insert(key, value);
    }
    pub fn reset(self: &mut Self) {
        self.preview_pairs.clear();
    }
    pub fn parse(self: &Self) -> String {
        serde_json::to_string_pretty(&self.preview_pairs).unwrap()
    }
}
