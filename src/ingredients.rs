use uuid::Uuid;
use std::{collections::{HashMap, HashSet}, hash::Hash};
use serde::{Serialize, Deserialize};

use crate::ingredients;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Quality {
    Low,
    Medium,
    High,
    Any
}

// Tag for types of ingredients. e.g. "Gin" or "sweet"
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IngredientTag {
    value: String
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Ingredient {
    name: String,
    quality: Quality,
    tags: Vec<IngredientTag>
}

// Struct for finding ingredients in an ingredient store
#[derive(Serialize, Deserialize)]
pub struct IngredientSelector {
    name: Option<String>,
    quality: Option<Quality>,
    tags: Option<Vec<IngredientTag>>
}

// A store of ingredients
#[derive(Serialize, Deserialize, Default)]
pub struct IngredientStore {
    ingredient_map: HashSet<Ingredient>,
    ingredient_tags: HashSet<IngredientTag>
}

impl IngredientStore {
    pub fn new() -> IngredientStore
    {
        IngredientStore::default()
    }

    pub fn select(&self, selector: IngredientSelector) -> Vec<Ingredient>
    {
        self.ingredient_map.iter().filter(|i| {
            match &selector.name {
                Some(n) => *n == i.name,
                None => true
            }
        }).filter(|i| {
            match &selector.quality {
                Some(q) => *q == i.quality,
                None => true
            }
        }).filter(|i| {
            match &selector.tags {
                Some(tags) => true,
                None => true
            }
        }).cloned().collect()
    }

    pub fn register_ingredient(&mut self, ingredient: Ingredient) {
        self.ingredient_map.insert(ingredient);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_registration() {
        let mut store = IngredientStore::new();
        assert_eq!(store.ingredient_map.len(), 0);
        let ingredient = Ingredient{ name: "foo".into(), quality: Quality::High, tags: vec![] };
        store.register_ingredient(ingredient.clone());
        assert_eq!(store.ingredient_map.len(), 1);
        store.register_ingredient(ingredient);
        assert_eq!(store.ingredient_map.len(), 1);
        let other_ingredient = Ingredient{ name: "foo2".into(), quality: Quality::High, tags: vec![] };
        store.register_ingredient(other_ingredient);
        assert_eq!(store.ingredient_map.len(), 2);
    }

    #[test]
    fn test_selector_quality() {
        let mut store = IngredientStore::new();
        let high_ingredient1 = Ingredient{ name: "high_1".into(), quality: Quality::High, tags: vec![] };
        store.register_ingredient(high_ingredient1);
        let high_ingredient2 = Ingredient{ name: "high_2".into(), quality: Quality::High, tags: vec![] };
        store.register_ingredient(high_ingredient2);
        let mid_ingredient1 = Ingredient{ name: "mid_1".into(), quality: Quality::Medium, tags: vec![] };
        store.register_ingredient(mid_ingredient1);
        let mid_ingredient2 = Ingredient{ name: "mid_2".into(), quality: Quality::Medium, tags: vec![] };
        store.register_ingredient(mid_ingredient2);
        let low_ingredient1 = Ingredient{ name: "low_1".into(), quality: Quality::Low, tags: vec![] };
        store.register_ingredient(low_ingredient1);
        let low_ingredient2 = Ingredient{ name: "low_2".into(), quality: Quality::Low, tags: vec![] };
        store.register_ingredient(low_ingredient2);
    }
}