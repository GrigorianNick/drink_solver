use crate::{builder::Builder, ingredient_selector_builder::IngredientSelectorBuilder, recipie::{Component, Measure}};

#[derive(Clone, Default)]
pub struct ComponentBuilder {
    pub selector: IngredientSelectorBuilder,
    pub measure: Measure,
}

/*impl Default for ComponentBuilder {
    fn default() -> Self {
        Self { selector: IngredientSelectorBuilder::default(), measure: Measure::default() }
    }
}*/

impl From<Component> for ComponentBuilder {
    fn from(value: Component) -> Self {
        ComponentBuilder { selector: value.ingredient.into(), measure: value.amount.clone() }
    }
}

impl Builder<Component> for ComponentBuilder {
    fn clear(&mut self) {
        self.selector = IngredientSelectorBuilder::default();
        self.measure = Measure::default();
    }

    fn build(&self) -> Component {
        Component { ingredient: self.selector.build(), amount: self.measure.clone() }
    }
}