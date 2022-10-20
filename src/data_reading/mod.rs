use serde::Deserialize;

use crate::{Constants, ParticleType};

pub mod compound;
pub mod element;

pub use compound::Compound;
pub use element::Element;

/// (energy in eV, stopping power in eV/m | 1/m)
pub type StoppingPower = Vec<(f32, f32)>;

#[derive(Debug, Clone)]
pub enum Substance {
    Element((Element, usize)),
    Compound(Compound),
}

impl Substance {
    pub fn symbol(&self) -> &String {
        match &self {
            Substance::Element(e) => &e.0.symbol,
            Substance::Compound(c) => &c.symbol,
        }
    }
    pub fn name(&self) -> &String {
        match &self {
            Substance::Element(e) => &e.0.name,
            Substance::Compound(c) => &c.name,
        }
    }
    /// in kg/m3
    pub fn density(&self) -> f32 {
        match &self {
            Substance::Element(e) => e.0.density,
            Substance::Compound(c) => c.density,
        }
    }

    pub fn stopping_power(&self, particle_type: &ParticleType) -> &StoppingPower {
        match &self {
            Substance::Element(e) => &e.0.stopping_powers[&particle_type],
            Substance::Compound(c) => &c.stopping_powers[&particle_type],
        }
    }

    /// if all required info is available for it to absorb radiation
    pub fn is_absorber(&self) -> bool {
        match &self {
            Substance::Element(e) => e.0.is_absorber,
            Substance::Compound(c) => c.is_absorber,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SubstanceIdentifier {
    Element(usize, usize),
    Compound(String),
}

impl SubstanceIdentifier {
    pub fn get(&self, constants: &Constants) -> Substance {
        match self {
            SubstanceIdentifier::Element(ref e, n) => {
                Substance::Element((constants.elements[&e].clone(), n.clone()))
            }
            SubstanceIdentifier::Compound(ref name) => {
                Substance::Compound(constants.compounds[name.as_str()].clone())
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StoppingPowerRow {
    /// in MeV
    pub energy: String,
    /// in MeV cm2/g
    pub stop_power: String,
}

#[derive(Debug, Deserialize)]
pub struct MassAttenuationCoefficientRow {
    /// in MeV
    pub energy: String,
    /// in cm2/g
    pub yp: String,
    /// in cm2/g
    pub yenp: String,
}

/// Parse numbers with scientific notation.
/// Will never fail, just return 0.
fn parse_num(num: &str) -> f32 {
    // scientific notation
    if !num.contains("E") {
        return num.parse().unwrap_or_else(|_| 0.0);
    } else {
        let num = num.split_once("E").unwrap();
        return num.0.parse().unwrap_or_else(|_| 0.0)
            * 10_f32.powi(num.1.parse().unwrap_or_else(|_| 0));
    }
}
