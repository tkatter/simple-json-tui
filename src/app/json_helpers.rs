pub fn create_array(values: Vec<serde_json::Value>) -> Vec<serde_json::Value> {
    let mut value_array: Vec<serde_json::Value> = Vec::new();

    for val in values.iter() {
        value_array.push(val.clone())
    }

    value_array
}

pub fn create_object(
    key: String,
    value: serde_json::Value,
) -> serde_json::Map<String, serde_json::Value> {
    let mut obj = serde_json::Map::new();

    obj.insert(key, value);

    obj
}
