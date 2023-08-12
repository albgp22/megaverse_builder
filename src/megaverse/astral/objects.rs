use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Blue,
    Red,
    Purple,
    White,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "blue" => Ok(Self::Blue),
            "red" => Ok(Self::Red),
            "purple" => Ok(Self::Purple),
            "white" => Ok(Self::White),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err(()),
        }
    }
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

impl AstralObject {
    pub fn build_from_string(row: u32, column: u32, description: String) -> Option<AstralObject> {
        if description.contains("POLYANET") {
            return Some(AstralObject::Polyanet { row, column });
        }
        if description.contains("COMETH") {
            return Some(AstralObject::Cometh {
                row,
                column,
                direction: Direction::from_str(description.as_str().split('_').next()?).unwrap(),
            });
        }
        if description.contains("SOLOON") {
            return Some(AstralObject::Soloon { 
                row,
                column,
                color: Color::from_str(description.as_str().split('_').next()?).unwrap(),
            });
        }
        None
    }
}
