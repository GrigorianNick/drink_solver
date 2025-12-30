use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};
use crate::{ingredient_store::{IngredientSelector, IngredientStore}, measure::Measure};

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