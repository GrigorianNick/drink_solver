use strum::{EnumIter, EnumString};
use uuid::Uuid;
use std::{collections::{HashMap, HashSet}, fmt, hash::Hash, path::PathBuf, str::FromStr};
use serde::{Serialize, Deserialize};

use crate::{ingredient, store::Store};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Hash, EnumString, EnumIter)]
pub enum Quality {
    Low,
    Medium,
    High,
    #[default]
    Any
}

/*impl FromStr for Quality {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Low" => Ok(Quality::Low),
        }
    }
}*/

impl fmt::Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            Quality::Low => "Low",
            Quality::Medium => "Medium",
            Quality::High => "High",
            Quality::Any => "Any",
        };
        write!(f, "{}", val)
    }
}

// Tag for types of ingredients. e.g. "Gin" or "sweet"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct IngredientTag {
    pub value: String
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Ingredient {
    pub name: String,
    pub quality: Quality,
    pub tags: Vec<IngredientTag>
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::ingredient_store::{IngredientSelector, IngredientStore};

    use super::*;

    #[test]
    fn test_selector_quality() {
        // Setup
        let mut store = IngredientStore::default();
        let high_ingredient1 = Ingredient{ name: "high_1".into(), quality: Quality::High, tags: vec![] };
        store.register(high_ingredient1);
        let high_ingredient2 = Ingredient{ name: "high_2".into(), quality: Quality::High, tags: vec![] };
        store.register(high_ingredient2);
        let mid_ingredient1 = Ingredient{ name: "mid_1".into(), quality: Quality::Medium, tags: vec![] };
        store.register(mid_ingredient1);
        let mid_ingredient2 = Ingredient{ name: "mid_2".into(), quality: Quality::Medium, tags: vec![] };
        store.register(mid_ingredient2);
        let low_ingredient1 = Ingredient{ name: "low_1".into(), quality: Quality::Low, tags: vec![] };
        store.register(low_ingredient1);
        let low_ingredient2 = Ingredient{ name: "low_2".into(), quality: Quality::Low, tags: vec![] };
        store.register(low_ingredient2);


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
        store.register(any);
        for qual in vec![Some(Quality::Any), Some(Quality::High), Some(Quality::Medium), Some(Quality::Low), None] {
            let selector = IngredientSelector{name: None, quality: qual, tags: None};
            let result = store.select(&selector);
            assert_eq!(result.len(), 1);
        }
        let high = Ingredient{ name: "high".into(), quality: Quality::High, tags: vec![]};
        store.register(high);
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
        store.register(dupe_1);
        store.register(dupe_2);
        store.register(novel);

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