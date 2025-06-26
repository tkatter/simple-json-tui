use super::editing_preview::UpdateMap;
use serde_json::Map;

#[derive(Default)]
pub struct ObjectValues {
    pub key: String,
    pub values: Map<String, serde_json::Value>,
}

impl ObjectValues {
    pub fn add_key(&mut self, key: &str) {
        self.key = key.to_string();
    }
    pub fn remove_entry(&mut self, key: &str) {
        self.values.remove(key);
    }
}

impl UpdateMap for ObjectValues {
    fn push(&mut self, key: &str, value: serde_json::Value) {
        self.values.insert(key.to_string(), value);
    }
    fn update_key(&mut self, key: &str, new_key: &str) {
        let values = self.values.remove(key).unwrap();
        self.push(new_key, values);
    }
    fn reset(&mut self) {
        self.values.clear();
    }
    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    fn parse(&self) -> String {
        serde_json::to_string_pretty(&self.values).unwrap()
    }
}
