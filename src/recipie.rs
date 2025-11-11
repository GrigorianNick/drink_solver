use crate::ingredients::{IngredientSelector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Recipie {
    ingredients: Vec<IngredientSelector>
}