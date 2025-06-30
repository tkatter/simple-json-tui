#[derive(Default)]
pub struct ArrayValues {
    pub values: Vec<serde_json::Value>,
}

impl ArrayValues {
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push_value(&mut self, value: serde_json::Value) {
        self.values.push(value);
    }
}
