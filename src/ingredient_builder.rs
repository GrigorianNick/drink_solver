use crate::{
    builder::Builder,
    ingredient::{Ingredient, IngredientTag, Quality},
};

#[derive(Clone, Default)]
pub struct IngredientBuilder {
    pub name: String,
    pub quality: Quality,
    pub tags: Vec<String>,
    pub stock: u16,
    pub is_liquor: bool,
}

impl From<Ingredient> for IngredientBuilder {
    fn from(value: Ingredient) -> Self {
        IngredientBuilder {
            name: value.name,
            quality: value.quality,
            tags: value.tags.iter().map(|t| t.value.clone()).collect(),
            stock: value.stock,
            is_liquor: value.is_liquor, 
        }
    }
}

impl Into<Ingredient> for IngredientBuilder {
    fn into(self) -> Ingredient {
        Ingredient {
            name: self.name.clone(),
            quality: self.quality,
            tags: self
                .tags
                .iter()
                .map(|t| IngredientTag { value: t.clone() })
                .collect(),
            stock: self.stock,
            is_liquor: self.is_liquor,
        }
    }
}

impl Builder<Ingredient> for IngredientBuilder {
    fn clear(&mut self) {
        self.name.clear();
        self.quality = Quality::Any;
        self.tags.clear();
    }

    fn build(&self) -> Ingredient {
        Ingredient {
            name: self.name.clone(),
            quality: self.quality,
            tags: self
                .tags
                .iter()
                .map(|t| IngredientTag { value: t.clone() })
                .collect(),
            stock: self.stock,
            is_liquor: self.is_liquor,
        }
    }
}
