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
    pub fn push(&mut self, key: String, value: serde_json::Value) {
        self.preview_pairs.insert(key, value);
    }
    pub fn reset(&mut self) {
        self.preview_pairs.clear();
    }
    pub fn parse(&self) -> String {
        serde_json::to_string_pretty(&self.preview_pairs).unwrap()
    }
    pub fn is_empty(&self) -> bool {
        self.preview_pairs.is_empty()
    }
    pub fn update_value(&mut self, key: String, new_value: serde_json::Value) {
        self.preview_pairs.insert(key, new_value);
    }
    pub fn update_key(&mut self, key: &String, new_key: String) {
        let values = self.preview_pairs.remove(key).unwrap();
        self.push(new_key, values);
    }

    /// Creates a new string with the given key.
    pub fn new_string(&mut self, key: String) {
        self.reset();
        self.push(key, serde_json::Value::String("".to_string()));
    }

    /// Creates a new array with the given key.
    pub fn new_array(&mut self, key: String) {
        self.reset();
        let empty_vec: Vec<serde_json::Value> = Vec::new();
        self.push(key, serde_json::Value::Array(empty_vec));
    }

    /// Creates a new object with the given key.
    pub fn new_object(&mut self, key: String) {
        self.reset();
        let empty_obj: Map<String, serde_json::Value> = Map::new();
        self.push(key, serde_json::Value::Object(empty_obj));
    }
}
