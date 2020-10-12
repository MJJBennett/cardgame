use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub ip: String,
}
