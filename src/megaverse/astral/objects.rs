
use serde::Serialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    fmt::{Debug},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum Color {
    Blue,
    Red,
    Purple,
    White,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
#[serde(untagged)]
pub enum AstralObject {
    Polyanet {
        row: u32,
        column: u32,
    },

    Soloon {
        row: u32,
        column: u32,
        color: Color,
    },

    Cometh {
        row: u32,
        column: u32,
        direction: Direction,
    },
}

impl AstralObject {
    pub fn json_with_additional_fields(&self, fields: HashMap<String, String>) -> String {
        let v = serde_json::to_value(self).unwrap();
        match v {
            Value::Object(m) => {
                let mut m = m;
                for (k, v) in fields {
                    m.insert(k.clone(), Value::String(v.clone()));
                }
                Value::Object(m)
            }
            v => v,
        }
        .to_string()
    }
}
