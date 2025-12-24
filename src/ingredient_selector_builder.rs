use crate::{builder::Builder, ingredient::{IngredientTag, Quality}, ingredient_store::IngredientSelector};

#[derive(Default, Clone)]
pub struct IngredientSelectorBuilder {
    pub name: String,
    pub quality: Option<Quality>,
    pub tags: Option<Vec<IngredientTag>>
}

impl From<IngredientSelector> for IngredientSelectorBuilder {
    fn from(value: IngredientSelector) -> Self {
        IngredientSelectorBuilder {
            name: value.name.unwrap_or_default(),
            quality: value.quality,
            tags: value.tags,
        }
    }
}

impl Builder<IngredientSelector> for IngredientSelectorBuilder {
    fn clear(&mut self) {
        self.name = String::default();
        self.quality = None;
        self.tags = None
    }

    fn build(&self) -> IngredientSelector {
        IngredientSelector {
            name: if self.name.is_empty() { None } else { Some(self.name.clone())},
            quality: self.quality,
            tags: self.tags.clone(),
        }
    }
}