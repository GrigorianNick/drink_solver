use core::fmt;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::ingredient_store::IngredientSelector;

#[derive(Serialize, Default, Deserialize, PartialEq, Clone, EnumIter)]
pub enum Measure {
    Oz(f32),
    Shot(f32),
    #[default]
    Taste
}

impl fmt::Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Measure::Oz(m) => write!(f, "{} oz", m),
            Measure::Shot(m) => write!(f, "{} shots", m),
            Measure::Taste => write!(f, "to taste"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Component {
    pub ingredient: IngredientSelector,
    pub amount: Measure
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Recipie {
    pub name: String,
    pub description: String,
    pub short_description: String,
    pub notes: String,
    pub components: Vec<Component>,
    pub instructions: Vec<String>
}

impl Recipie {
}