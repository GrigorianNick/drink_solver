use crate::builder::Builder;
use crate::recipie::{Component, Recipie};

#[derive(Clone, Default)]
pub struct RecipieBuilder {
    pub name: String,
    pub description: String,
    pub short_description: String,
    pub notes: String,
    pub components: Vec<Component>,
    pub instructions: Vec<String>,
}

impl From<Recipie> for RecipieBuilder {
    fn from(value: Recipie) -> Self {
        RecipieBuilder {
            name: value.name.clone(),
            description: value.description.clone(),
            short_description: value.short_description.clone(),
            notes: value.notes.clone(),
            components: value.components.clone(),
            instructions: value.instructions.clone(),
        }
    }
}

impl Builder<Recipie> for RecipieBuilder {
    fn build(&self) -> Recipie {
        let mut recipie = Recipie::default();
        recipie.name = self.name.clone();
        recipie.short_description = self.short_description.clone();
        recipie.description = self.description.clone();
        recipie.notes = self.notes.clone();
        recipie.components = self.components.clone();
        recipie.instructions = self.instructions.clone();
        recipie
    }

    fn clear(&mut self) {
        self.name.clear();
        self.description.clear();
        self.short_description.clear();
        self.notes.clear();
        self.components.clear();
    }
}

impl RecipieBuilder {
    pub fn new() -> RecipieBuilder {
        RecipieBuilder::default()
    }
}
