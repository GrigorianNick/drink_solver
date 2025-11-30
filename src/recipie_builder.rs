use crate::recipie::Recipie;

enum MeasureBuilder {
    Num,
    // (Numerator, Denominator)
    Fraction(u32, u32)
}

impl MeasureBuilder {

}

#[derive(Clone, Default)]
pub struct RecipieBuilder {
    pub name: String
}

impl RecipieBuilder {
    pub fn new() -> RecipieBuilder {
        RecipieBuilder::default()
    }

    pub fn new_from(base: &Recipie) -> RecipieBuilder {
        let me = RecipieBuilder::new();
        me
    }

    pub fn build(&self) -> Recipie {
        let mut recipie = Recipie::default();
        recipie.name = self.name.clone();
        recipie
    }
}