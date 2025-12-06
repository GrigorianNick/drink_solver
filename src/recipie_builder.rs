use crate::recipie::Recipie;
use crate::builder::Builder;

enum MeasureBuilder {
    Num,
    // (Numerator, Denominator)
    Fraction(u32, u32)
}

impl MeasureBuilder {

}

#[derive(Clone, Default)]
pub struct RecipieBuilder {
    pub name: String,
    pub description: String,
    pub short_description: String,
    pub notes: String
}

impl Builder<Recipie> for RecipieBuilder {
    fn new_from(base: &Recipie) -> Self {
        RecipieBuilder {
            name: base.name.clone(),
            description: base.description.clone(),
            short_description: base.short_description.clone(),
            notes: base.notes.clone()
        }
    }

    fn build(&self) -> Recipie {
        let mut recipie = Recipie::default();
        recipie.name = self.name.clone();
        recipie.short_description = self.short_description.clone();
        recipie
    }
}

impl RecipieBuilder {
    pub fn new() -> RecipieBuilder {
        RecipieBuilder::default()
    }
}