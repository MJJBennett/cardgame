// This module represents the actual game.
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use structopt::StructOpt;
use std::fs::File;
use std::io::{self, BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub enum Target {
    Other,
    Self,
    Any,
    Bank,
    Deck,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Money(Target, i16, bool/*From, To*/),
    Damage(Target, i16),     
    Unblockable, // prevents blocking
    Replace(bool/*Current, initial*/),
    Swap(Target, i8),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Effect {
    cost: Optional<i16>,
    does: Vec<Action>,
    blocks: Vec<String>, // regexes, basically lol
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    // Is the game currently running?
    pub running: bool,
};

// We use ser/de here for sending along the
// wire (to client user machines) for simplicity.
// However, we actually deserialize initially from
// a script file.
#[derive(Debug, Serialize, Deserialize)]
pub struct GameRules {
    pub effects: HashMap<String, Effect>,
}

pub struct Game {
    
}

pub impl GameRules {
    pub fn from(filename: String) -> GameRules {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

    }
}
