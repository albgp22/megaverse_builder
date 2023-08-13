/*
    Megaverse-specific types
*/

use log::error;
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

// Constructor /*air quotes*/ from string. Useful for parsing objects from the REST API.
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

/*
Dumps the astral object to JSON string, allowing to specify extra fields if necessary.
*/
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

    #[allow(dead_code)]
    pub fn json(&self) -> String {
        self.json_with_additional_fields(HashMap::new())
    }
}

// Builder from application-specific string.
impl AstralObject {
    pub fn build_from_string(row: u32, column: u32, description: String) -> Option<AstralObject> {
        if description.contains("POLYANET") {
            return Some(AstralObject::Polyanet { row, column });
        }
        if description.contains("COMETH") {
            let direction = match Direction::from_str(description.as_str().split('_').next()?) {
                Ok(direction) => direction,
                Err(e) => {
                    error!(
                        "Direction could not be parsed from {description}: {e:?}. Skipping object."
                    );
                    return None;
                }
            };
            return Some(AstralObject::Cometh {
                row,
                column,
                direction,
            });
        }
        if description.contains("SOLOON") {
            let color = match Color::from_str(description.as_str().split('_').next()?) {
                Ok(color) => color,
                Err(e) => {
                    error!("Color could not be parsed from {description}: {e:?}. Skipping object.");
                    return None;
                }
            };
            return Some(AstralObject::Soloon { row, column, color });
        }
        None
    }
}
