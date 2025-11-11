
use std::{collections::HashMap, default, fs::File, path::PathBuf};

use serde::{Serialize, Deserialize};

use crate::{ingredients::IngredientSelector, recipie::Recipie};

#[derive(Serialize, Deserialize, Default)]
pub struct RecipieStore {
    recipies: HashMap<uuid::Uuid, Recipie>
}

impl RecipieStore {
    pub fn new() -> RecipieStore {
        match dirs::config_local_dir() {
            Some(path) => Self::from_config(path),
            _ => Self::from_config(PathBuf::from("."))
        }
    }

    pub fn from_config(config_dir: PathBuf) -> RecipieStore {
        match File::open(config_dir.join("recipies.json"))
        {
            Ok(fs) => serde_json::from_reader(fs).unwrap_or_default(),
            Err(_) => RecipieStore::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vacuous() {
        assert!(true);
    }
}