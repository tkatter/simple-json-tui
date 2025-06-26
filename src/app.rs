pub mod screens;
use ratatui::crossterm::event::{KeyEvent, KeyModifiers};
// use json_helpers::{create_array, create_object};
use screens::{editing_preview::EditingPreview, selection::SelectionScreen};
use serde_json::{Map, Number};
// use serde_json::{json, to_value};
use std::collections::HashMap;
// use std::{fs::File, io::BufWriter};

#[derive(Default)]
pub enum CurrentScreen {
    Editing(ValueType),
    #[default]
    Main,
    Quitting,
    Selection,
    Start,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub enum ValueType {
    Array,
    Bool,
    Number,
    Object,
    #[default]
    String,
}

// pub enum ObjectTypes {

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

    pub fn reset(&mut self) {
        self.values = Vec::new()
    }
}

#[derive(Default)]
pub struct ObjectValues {
    pub key: String,
    pub values: Map<String, serde_json::Value>,
}

impl ObjectValues {
    pub fn add_key(&mut self, key: &str) {
        self.key = key.to_string();
    }
    pub fn push(&mut self, key: String, value: serde_json::Value) {
        self.values.insert(key, value);
    }
}

#[derive(Default)]
pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub value_type: ValueType,
    pub pairs: HashMap<String, serde_json::Value>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // The current screen the user is looking at, and will later determin what to render
    pub currently_editing: Option<CurrentlyEditing>, // The optional state containing what the user is currently editing of the key or value, if not editing then will be None
    pub editing_object: bool,
    pub array_values: ArrayValues,
    pub object_values: ObjectValues,
    pub selection_screen: SelectionScreen,
    pub editing_preview: EditingPreview,
    // TODO: Build Mutex file/thread/async handling flow
    // pub tmp_path: BufWriter<File>
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            value_type: ValueType::String,
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Start,
            currently_editing: None,
            editing_object: false,
            object_values: ObjectValues::default(),
            array_values: ArrayValues::default(),
            selection_screen: SelectionScreen::default(),
            editing_preview: EditingPreview::default(),
        }
    }

    pub fn store_array_values(&mut self) {
        let input = &self.value_input;
        let value = serde_json::to_value(input).unwrap();

        self.array_values.push_value(value);

        if self.editing_object {
            self.add_object_value(Some(serde_json::Value::Array(
                self.array_values.values.to_owned(),
            )));
        } else {
            self.editing_preview.update_value(
                &self.key_input,
                serde_json::Value::Array(self.array_values.values.to_owned()),
            );
        }
    }

    /// Adds a key value pair to the temporary object store.
    /// Also updates the editing preview.
    ///
    /// ---
    /// `value = Option<serde_json::Value>`
    /// ---
    ///
    /// If `None` is passed as `value`, will use the `value_input`
    pub fn add_object_value(&mut self, value: Option<serde_json::Value>) {
        let key = self.key_input.to_string();

        if let Some(value) = value {
            self.object_values.push(key, value);
        } else {
            self.object_values.push(
                key,
                serde_json::to_value(self.value_input.to_string()).unwrap(),
            );
        }

        self.editing_preview.push(
            &self.object_values.key,
            serde_json::Value::Object(self.object_values.values.to_owned()),
        );
    }

    pub fn del_char(&mut self) {
        if let Some(editing) = &self.currently_editing {
            match editing {
                CurrentlyEditing::Key => {
                    self.key_input.pop();
                }
                CurrentlyEditing::Value => {
                    self.value_input.pop();
                }
            }
        }
    }

    pub fn push_char(&mut self, key: &KeyEvent, value: char) {
        if let Some(editing) = &self.currently_editing {
            // Need this to avoid adding characters when CTRL is pressed
            if !key.modifiers.contains(KeyModifiers::CONTROL) {
                match editing {
                    CurrentlyEditing::Key => {
                        self.key_input.push(value);
                    }
                    CurrentlyEditing::Value => {
                        self.value_input.push(value);
                    }
                }
            }
        }
    }

    pub fn save_key_value(&mut self) {
        let mut key: &str = &self.key_input;
        let value = match &self.value_type {
            ValueType::String => serde_json::Value::String(self.value_input.to_string()),
            ValueType::Array => {
                let values: Vec<serde_json::Value> = self.array_values.values.to_owned();
                serde_json::Value::Array(values)
            }
            ValueType::Object => {
                key = &self.object_values.key;
                let mut new_map: Map<String, serde_json::Value> = Map::new();
                new_map.append(&mut self.object_values.values);
                serde_json::Value::Object(new_map)
            }
            _ => {
                let number_val: Number = self
                    .value_input
                    .clone()
                    .parse()
                    .expect("Failed to parse into `Number` - Handle Error");
                // json!(number_val)
                serde_json::Value::Number(number_val)
            }
        };

        if self.editing_object {
            // Add new value before moving the map
            // cant use `add_object_value` because of borrowing
            self.object_values.push(key.to_string(), value);

            // Update the stored object values && editing preview
            self.editing_preview.push(
                &self.object_values.key,
                serde_json::Value::Object(self.object_values.values.to_owned()),
            );
            self.key_input = String::new();
            self.value_input = String::new();
            self.current_screen = CurrentScreen::Main;
            self.currently_editing = None;
            self.array_values = ArrayValues::default();
        } else {
            self.pairs.insert(key.to_string(), value);
        }
        // self.handle_escape();
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn handle_escape(&mut self) {
        *self = Self {
            pairs: self.pairs.to_owned(),
            ..Self::default()
        };
    }

    pub fn toggle_value_type(&mut self) {
        let current_type = &self.value_type;

        match *current_type {
            ValueType::String => {
                self.value_type = ValueType::Number;
                self.current_screen = CurrentScreen::Editing(ValueType::Number);
            }
            ValueType::Number => {
                self.value_type = ValueType::Bool;
                self.current_screen = CurrentScreen::Editing(ValueType::Bool);
            }
            ValueType::Bool => {
                self.value_type = ValueType::Object;
                self.current_screen = CurrentScreen::Editing(ValueType::Object)
            }
            ValueType::Object => {
                self.value_type = ValueType::Array;
                self.current_screen = CurrentScreen::Editing(ValueType::Array)
            }
            ValueType::Array => {
                self.value_type = ValueType::String;
                self.current_screen = CurrentScreen::Editing(ValueType::String);
                self.editing_preview.new_string(&self.key_input);
            }
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string_pretty(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}
