use serde_json::Map;

use crate::traits::UpdateMap;

pub struct EditingPreview {
    pub values: Map<String, serde_json::Value>,
}

impl Default for EditingPreview {
    fn default() -> Self {
        EditingPreview { values: Map::new() }
    }
}

impl UpdateMap for EditingPreview {
    fn update_key(&mut self, key: &str, new_key: &str) {
        let values = self.values.remove(key).unwrap();
        self.push(new_key, values);
    }
    fn push(&mut self, key: &str, value: serde_json::Value) {
        self.values.insert(key.to_string(), value);
    }
    fn reset(&mut self) {
        self.values.clear();
    }
    fn parse(&self) -> String {
        serde_json::to_string_pretty(&self.values).unwrap()
    }
    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
