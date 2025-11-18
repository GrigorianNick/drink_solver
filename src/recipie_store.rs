
use std::{collections::HashMap, default, fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{ingredients::IngredientSelector, recipie::Recipie};

#[derive(Serialize, Deserialize, Default)]
pub struct RecipieStore {
    recipies: HashMap<uuid::Uuid, Recipie>,
    config: Option<PathBuf>
}

impl RecipieStore {
    // Tries to deserialize from a common json file, otherwise it only populates in memory
    pub fn new() -> RecipieStore {
        let cfg_dir = dirs::config_local_dir().unwrap_or(PathBuf::from("."));
        match Self::from_config(cfg_dir)
        {
            Ok(store) => store,
            Err(_) => RecipieStore::default()
        }
    }

    // Loads from the specified config
    pub fn from_config(config_dir: PathBuf) -> Result<RecipieStore, Box<dyn std::error::Error>> {
        let path = config_dir.join("recipies.json");
        if std::fs::exists(&path)? {
            let cfg_file = File::open(path)?;
            let cfg = BufReader::new(cfg_file);
            let store = serde_json::from_reader(cfg)?;
            Ok(store)
        }
        else {
            std::fs::File::create_new(&path)?;
            let mut store = RecipieStore::default();
            store.config = Some(path);
            Ok(store)
        }
    }

    // Saves to whatever config this was loaded from
    pub fn save(&self) -> bool {
        match &self.config {
            Some(path) => self.save_to(path),
            _ => false
        }
    }

    // Saves to a specific file
    pub fn save_to(&self, path: &PathBuf) -> bool {
        match File::create(path) {
            Ok(file) => serde_json::to_writer(file, self).is_ok(),
            _ => false
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