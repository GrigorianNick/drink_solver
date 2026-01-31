use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::{
    ingredient::{Ingredient, IngredientTag, Quality},
    store::Store,
};

// Struct for finding ingredients in an ingredient store
#[derive(Serialize, Default, Deserialize, Clone)]
pub struct IngredientSelector {
    pub name: Option<String>,
    pub quality: Option<Quality>,
    pub tags: Option<Vec<IngredientTag>>,
    pub in_stock: Option<bool>,
    pub is_liquor: Option<bool>,
}

// A store of ingredients
#[derive(Serialize, Deserialize, Default)]
pub struct IngredientStore {
    ingredient_map: HashMap<uuid::Uuid, Ingredient>,
    ingredient_tags: HashSet<IngredientTag>,
    config_path: Option<PathBuf>,
}

impl Store<Ingredient> for IngredientStore {
    fn get_json_name() -> std::path::PathBuf {
        "ingredient.json".into()
    }

    fn set_config_path(&mut self, path: std::path::PathBuf) {
        self.config_path = Some(path)
    }

    fn get_config_path(&self) -> Option<std::path::PathBuf> {
        self.config_path.clone()
    }

    fn register(&mut self, entry: Ingredient) -> uuid::Uuid {
        let id = uuid::Uuid::new_v4();
        for tag in &entry.tags {
            self.ingredient_tags.insert(tag.clone());
        }
        self.ingredient_map.insert(id, entry);
        id
    }

    fn deregister(&mut self, id: uuid::Uuid) -> bool {
        self.ingredient_map.remove(&id).is_some()
    }

    fn get_entries(&self) -> Vec<Ingredient> {
        self.ingredient_map.values().cloned().collect()
    }

    fn get_entries_mut(&mut self) -> Vec<&mut Ingredient> {
        self.ingredient_map.values_mut().collect()
    }

    fn get_entry(&self, id: uuid::Uuid) -> Option<Ingredient> {
        match self.ingredient_map.get(&id) {
            Some(entry) => Some(entry.clone()),
            None => None,
        }
    }

    fn get_entry_mut(&mut self, id: uuid::Uuid) -> Option<&mut Ingredient> {
        match self.ingredient_map.get_mut(&id) {
            Some(entry) => Some(entry),
            None => None,
        }
    }
}

impl IngredientStore {
    /*pub fn new() -> IngredientStore
    {
        IngredientStore::default()
    }*/

    pub fn select(&self, selector: &IngredientSelector) -> Vec<Ingredient> {
        self.ingredient_map
            .values()
            .filter(|i| match &selector.name {
                Some(n) => *n == i.name,
                None => true,
            })
            .filter(|i| match selector.quality {
                Some(q) => q == Quality::Any || i.quality == Quality::Any || q == i.quality,
                None => true,
            })
            .filter(|i| match &selector.tags {
                Some(tags) => {
                    for tag in tags {
                        if !i.tags.contains(tag) {
                            return false;
                        }
                    }
                    return true;
                }
                None => true,
            })
            .filter(|i| match selector.in_stock {
                Some(true) => i.stock > 0,
                Some(false) => i.stock == 0,
                None => true,
            })
            .filter(|i| match selector.is_liquor {
                Some(l) => l == i.is_liquor,
                None => true
            })
            .cloned()
            .collect()
    }

    pub fn get_ingredient_names(&self) -> Vec<String> {
        return self
            .ingredient_map
            .values()
            .map(|i| i.name.clone())
            .collect();
    }

    pub fn get_tags(&self) -> Vec<IngredientTag> {
        return self.ingredient_tags.iter().cloned().collect();
    }

    pub fn get_ingredient(&self, name: &String) -> Option<Ingredient> {
        self.ingredient_map
            .values()
            .find(|&i| &i.name == name)
            .cloned()
    }

    pub fn get_ingredient_entries(&mut self) -> Vec<(uuid::Uuid, &mut Ingredient)> {
        self.ingredient_map
            .iter_mut()
            .map(|(id, i)| (id.clone(), i))
            .collect()
    }
}
