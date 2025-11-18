use crate::ingredients::{IngredientSelector};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
enum Measure {
    oz(f32),
    shot(f32),
    taste
}

#[derive(Serialize, Deserialize)]
struct Component {
    ingredient: IngredientSelector,
    amount: Measure
}

#[derive(Serialize, Deserialize)]
pub struct Recipie {
    name: String,
    components: Vec<Component>
}

impl Recipie {

}