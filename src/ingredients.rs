use uuid::Uuid;
use std::{collections::{HashMap, HashSet}, hash::Hash};
use serde::{Serialize, Deserialize};

use crate::ingredients;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
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

    pub fn select(&self, selector: &IngredientSelector) -> Vec<Ingredient>
    {
        self.ingredient_map.iter().filter(|i| {
            match &selector.name {
                Some(n) => *n == i.name,
                None => true
            }
        }).filter(|i| {
            match selector.quality {
                Some(q) => q == Quality::Any || i.quality == Quality::Any || q == i.quality,
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
    use std::vec;

    use super::*;

    #[test]
    fn test_registration() {
        let mut store = IngredientStore::default();
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
        // Setup
        let mut store = IngredientStore::default();
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


        // Specific quality check
        for qual in vec![Quality::Low, Quality::Medium, Quality::High] {
            let selector = IngredientSelector{ name: None, quality: Some(qual), tags: None };
            let result = store.select(&selector);
            assert_eq!(result.len(), 2);
        }

        // Quality::Any or None should fetch everything
        let selector = IngredientSelector{ name: None, quality: Some(Quality::Any), tags: None };
        let result = store.select(&selector);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_selector_any_quality() {
        let mut store = IngredientStore::default();
        let any = Ingredient{ name: "any".into(), quality: Quality::Any, tags:vec![]};
        store.register_ingredient(any);
        for qual in vec![Some(Quality::Any), Some(Quality::High), Some(Quality::Medium), Some(Quality::Low), None] {
            let selector = IngredientSelector{name: None, quality: qual, tags: None};
            let result = store.select(&selector);
            println!("{:?}", qual);
            assert_eq!(result.len(), 1);
        }
        let high = Ingredient{ name: "high".into(), quality: Quality::High, tags: vec![]};
        store.register_ingredient(high);
        for qual in vec![Some(Quality::Any), Some(Quality::High), None] {
            let selector = IngredientSelector{name: None, quality: qual, tags: None};
            let result = store.select(&selector);
            assert_eq!(result.len(), 2);
        }
    }

    #[test]
    fn test_selector_name() {
        // Setup
        let mut store = IngredientStore::default();
        let dupe_1 = Ingredient{ name: "dupe".into(), quality: Quality::Low, tags: vec![]};
        let dupe_2 = Ingredient{ name: "dupe".into(), quality: Quality::High, tags: vec![]};
        let novel = Ingredient{ name: "novel".into(), quality: Quality::Any, tags: vec![]};
        store.register_ingredient(dupe_1);
        store.register_ingredient(dupe_2);
        store.register_ingredient(novel);

        // Check specific names
        let dupe_selector = IngredientSelector{ name: Some("dupe".into()), quality: None, tags: None};
        let dupe_result = store.select(&dupe_selector);
        assert_eq!(dupe_result.len(), 2);
        let novel_selector = IngredientSelector{ name: Some("novel".into()), quality: None, tags: None};
        let novel_result = store.select(&novel_selector);
        assert_eq!(novel_result.len(), 1);

        // Check that None gets everything
        let none_selector = IngredientSelector{ name: None, quality: None, tags: None};
        let none_result = store.select(&none_selector);
        assert_eq!(none_result.len(), 3);

        // Check a bad name gets nothing
        let bad_selector = IngredientSelector{ name: Some("fake".into()), quality: None, tags: None};
        let bad_result = store.select(&bad_selector);
        assert_eq!(bad_result.len(), 0);
    }
}