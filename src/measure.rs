use std::fmt;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Serialize, Default, Deserialize, PartialEq, Clone, EnumIter)]
pub enum Measure {
    Oz(f32),
    Ml(f32),
    Shot(f32),
    Liter(f32),
    Handle(f32),
    Dash(f32),
    Teaspoon(f32),
    Tablespoon(f32),
    #[default]
    Taste,
}

impl fmt::Display for Measure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Measure::Oz(m) => write!(f, "{} oz", m),
            Measure::Ml(m) => write!(f, "{} ml", m),
            Measure::Shot(m) => write!(f, "{} shots", m),
            Measure::Liter(m) => write!(f, "{} liters", m),
            Measure::Handle(m) => write!(f, "{} handles", m),
            Measure::Dash(m) => write!(f, "{} dashes", m),
            Measure::Teaspoon(m) => write!(f, "{} teaspoons", m),
            Measure::Tablespoon(m) => write!(f, "{} tablespoons", m),
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
            Measure::Dash(m) => Measure::Oz(0.1),
            Measure::Teaspoon(m) => Measure::Ml(4.92892 * m).to_oz(),
            Measure::Tablespoon(m) => Measure::Teaspoon(3.0 * m).to_oz(),
            Measure::Taste => Measure::Oz(0.1),
        }
    }
}
