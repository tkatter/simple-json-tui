#![allow(unused)]
mod json_helpers;
mod selection_screen;
use json_helpers::{create_array, create_object};
use selection_screen::SelectionScreen;
use serde_json::{Number, json, to_value};
use std::collections::HashMap;

pub enum EditingScreens {
    Default,
    Object,
    Array,
}

pub enum CurrentScreen {
    Main,
    Selection,
    Editing(EditingScreens),
    Quitting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ValueType {
    String,
    Number,
    Bool,
    Object,
    Array,
}

pub struct ArrayValues {
    pub values: Vec<serde_json::Value>,
}

impl ArrayValues {
    pub fn push_value(&mut self, value: serde_json::Value) {
        self.values.push(value);
    }

    pub fn reset(&mut self) {
        self.values = Vec::new()
    }
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub value_type: ValueType,
    pub pairs: HashMap<String, serde_json::Value>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // The current screen the user is looking at, and will later determin what to render
    pub currently_editing: Option<CurrentlyEditing>, // The optional state containing what the user is currently editing of the key or value, if not editing then will be None
    pub array_values: ArrayValues,
    pub selection_screen: SelectionScreen,
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            value_type: ValueType::String,
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            array_values: ArrayValues { values: Vec::new() },
            selection_screen: SelectionScreen::default(),
        }
    }

    pub fn store_array_values(&mut self) {
        let input = self.value_input.clone();
        let value = serde_json::to_value(input).unwrap();

        self.array_values.push_value(value);
    }

    pub fn save_key_value(&mut self) {
        let key = self.key_input.clone();
        let value = match self.value_type {
            ValueType::String => serde_json::Value::String(self.value_input.clone()),
            ValueType::Array => {
                let values: Vec<serde_json::Value> = self.array_values.values.clone();
                serde_json::Value::Array(values)
            }
            // ValueType::Object => {
            //     let pair = self.value_input.clone().split_once('=').unwrap();
            //     let (key, val) = pair.to_owned();
            //     let obj =
            //         create_object(key.to_string(), serde_json::Value::String(val.to_string()));
            //     serde_json::Value::Object(obj)
            // }
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
        // For Array and HashMap --> Create a Vec<Value> | HashMap<Value, Value> and push K: V
        // pairs to it!!

        self.pairs.insert(key, value);

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
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

    pub fn toggle_value_type(&mut self) {
        let current_type = &self.value_type;
        let types: Vec<ValueType> = vec![
            ValueType::String,
            ValueType::Number,
            ValueType::Bool,
            ValueType::Object,
            ValueType::Array,
        ];

        match *current_type {
            ValueType::String => self.value_type = ValueType::Number,
            ValueType::Number => self.value_type = ValueType::Bool,
            ValueType::Bool => {
                self.value_type = ValueType::Object;
                self.current_screen = CurrentScreen::Editing(EditingScreens::Object)
            }
            ValueType::Object => {
                self.value_type = ValueType::Array;
                self.current_screen = CurrentScreen::Editing(EditingScreens::Array)
            }
            ValueType::Array => self.value_type = ValueType::String,
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}
