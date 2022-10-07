use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Coordinates {
    pub x: u8,
    pub y: u8,
}
impl Coordinates {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    pub fn index(&self) -> usize {
        (self.x * 3 + self.y).into()
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Cell {
    pub coordinates: Coordinates,
    pub player: Option<Addr>,
    pub sign: String,
}
impl Cell {
    pub fn new(coordinates: Coordinates) -> Self {
        Self {
            coordinates,
            player: None,
            sign: "".to_string(),
        }
    }
    pub fn restart(&self) -> Self {
        Self {
            coordinates: self.coordinates.clone(),
            player: None,
            sign: "".to_string(),
        }
    }
}
