use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Site {
    pub name: String,
    pub buildings: Vec<Building>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Building {
    pub name: String,
    pub levels: Vec<Level>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Level {
    pub name: String,
}
