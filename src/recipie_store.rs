
use std::{collections::HashMap, default, fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{ingredient::IngredientSelector, recipie::Recipie, store::Store};

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

    pub fn get_recipie_entries(&self) -> Vec<(uuid::Uuid, Recipie)> {
        self.recipies.iter().map(|(u, r)| (u.clone(), r.clone())).collect()
    }

    pub fn get_recipie(&self, id: uuid::Uuid) -> Option<Recipie> {
        match self.recipies.get(&id) {
            Some(r) => Some(r.clone()),
            None => None,
        }
    }

    pub fn get_recipie_mut(&mut self, id: uuid::Uuid) -> Option<&mut Recipie> {
        self.recipies.get_mut(&id)
    }

    pub fn get_recipies(&self) -> Vec<Recipie> {
        self.recipies.values().cloned().collect()
    }
}

impl Store<Recipie> for RecipieStore {
    fn get_json_name() -> PathBuf {
        "recipie.json".into()
    }

    fn set_config_path(&mut self, path: PathBuf) {
        self.config = Some(path)
    }

    fn get_config_path(&self) -> Option<PathBuf> {
        self.config.clone()
    }

    fn register(&mut self, entry: Recipie) -> uuid::Uuid {
        let id = uuid::Uuid::new_v4();
        self.recipies.insert(id, entry);
        id
    }
}

#[cfg(test)]
mod tests {
    use crate::ingredient::Ingredient;

    use super::*;

    #[test]
    fn test_recipie_mut() {
        let mut store = RecipieStore::default();
        let r = Recipie::default();
        let id = store.register(r.clone());
        {
            let mut_r_opt = store.get_recipie_mut(id);
            assert!(mut_r_opt.is_some());
            let mut_r = mut_r_opt.unwrap();
            mut_r.name = "New Name".into();
            mut_r.short_description = "New Short Description".into();
            mut_r.description = "New Description".into();
            mut_r.notes = "New Notes".into();
        }
        let new_r_opt = store.get_recipie(id);
        assert!(new_r_opt.is_some());
        let new_r = new_r_opt.unwrap();
        assert_ne!(r.name, new_r.name);
        assert_ne!(r.description, new_r.description);
        assert_ne!(r.short_description, new_r.short_description);
        assert_ne!(r.notes, new_r.notes);
    }
}