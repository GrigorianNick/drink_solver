use uuid::Uuid;
use std::collections::{HashMap, HashSet};

use crate::ingredients;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Quality {
    Low,
    Medium,
    High,
    Any
}

// Tag for types of ingredients. e.g. "Gin" or "sweet"
#[derive(Clone)]
pub struct IngredientTag {
    value: String
}

#[derive(Clone)]
pub struct Ingredient {
    name: String,
    quality: Quality,
    tags: Vec<IngredientTag>
}

// Struct for finding ingredients in an ingredient store
pub struct IngredientSelector {
    name: Option<String>,
    quality: Option<Quality>,
    tags: Option<Vec<IngredientTag>>
}

pub struct IngredientStore {
    ingredient_map: HashMap<Uuid, Ingredient>,
    ingredient_tags: HashSet<IngredientTag>
}

impl IngredientStore {
    pub fn new() -> IngredientStore
    {
        IngredientStore { ingredient_map: HashMap::new(), ingredient_tags: HashSet::new() }
    }

    pub fn select(self, selector: IngredientSelector) -> Vec<Ingredient>
    {
        self.ingredient_map.values().into_iter().filter(|i| {
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
}