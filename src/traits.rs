use serde_json::Map;

pub trait UpdateMap {
    fn update_key(&mut self, key: &str, new_key: &str);
    fn push(&mut self, key: &str, value: serde_json::Value);
    fn reset(&mut self);
    fn parse(&self) -> String;
    fn is_empty(&self) -> bool;

    /// Creates a new string with the given key.
    fn new_string(&mut self, key: &str, reset: bool) {
        if reset {
            self.reset();
        }

        self.push(key, serde_json::Value::String("".to_string()));
    }

    /// Creates a new array with the given key.
    fn new_array(&mut self, key: &str, reset: bool) {
        if reset {
            self.reset();
        }

        let empty_vec: Vec<serde_json::Value> = Vec::new();
        self.push(key, serde_json::Value::Array(empty_vec));
    }

    /// Creates a new object with the given key.
    fn new_object(&mut self, key: &str, reset: bool) {
        if reset {
            self.reset();
        }

        let empty_obj: Map<String, serde_json::Value> = Map::new();
        self.push(key, serde_json::Value::Object(empty_obj));
    }

    fn new_bool(&mut self, key: &str, reset: bool) {
        if reset {
            self.reset();
        }

        self.push(key, serde_json::Value::Bool(true));
    }

    fn new_number(&mut self, key: &str, reset: bool) {
        if reset {
            self.reset();
        }

        self.push(key, serde_json::Value::Number(0.into()));
    }
}
