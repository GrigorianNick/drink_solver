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
    Part(f32),
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
            Measure::Part(m) => write!(f, "{} parts", m),
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
            Measure::Part(m) => Measure::Oz(m),
            Measure::Taste => Measure::Oz(0.1),
        }
    }
}
