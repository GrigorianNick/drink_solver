use core::fmt;

use crate::ingredient::{IngredientSelector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub enum Measure {
    Oz(f32),
    Shot(f32),
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