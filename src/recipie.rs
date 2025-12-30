use core::fmt;
use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::ingredient_store::{IngredientSelector, IngredientStore};

#[derive(Serialize, Default, Deserialize, PartialEq, Clone, EnumIter)]
pub enum Measure {
    Oz(f32),
    Ml(f32),
    Shot(f32),
    Liter(f32),
    Handle(f32),
    #[default]
    Taste
}

impl fmt::Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Measure::Oz(m) => write!(f, "{} oz", m),
            Measure::Ml(m) => write!(f, "{} ml", m),
            Measure::Shot(m) => write!(f, "{} shots", m),
            Measure::Liter(m) => write!(f, "{} liters", m),
            Measure::Handle(m) => write!(f, "{} handles", m),
            Measure::Taste => write!(f, "to taste"),
        }
    }
}

impl Measure {
    pub fn to_oz(self) -> Measure {
        match self {
            Measure::Oz(m) => Measure::Oz(m),
            Measure::Ml(m) => Measure::Oz(0.033814 * m),
            Measure::Shot(m) => Measure::Oz(1.5 * m),
            Measure::Liter(m) => Measure::Oz(33.814 * m),
            Measure::Handle(m) => Measure::Liter(1.5 * m).to_oz(),
            Measure::Taste => Measure::Oz(0.1),
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
    pub fn can_make(&self, store: Rc<RefCell<IngredientStore>>) -> bool{
        self.components.iter().cloned().all(|mut c| {
            c.ingredient.in_stock = Some(true);
            !store.borrow().select(&c.ingredient).is_empty()
    })
    }
}