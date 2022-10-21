use std::sync::Arc;

use serde::Deserialize;

use crate::ParticleType;

pub mod compound;
pub mod element;

pub use compound::Compound;
pub use element::Element;

/// (energy in eV, stopping power in eV/m | 1/m)
pub type StoppingPower = Vec<(f32, f32)>;

#[derive(Debug, Clone)]
pub enum Substance {
    Element(Arc<Element>, usize),
    Compound(Arc<Compound>),
}

impl Substance {
    pub fn symbol(&self) -> &String {
        match &self {
            Substance::Element(e, _) => &e.symbol,
            Substance::Compound(c) => &c.symbol,
        }
    }
    pub fn name(&self) -> &String {
        match &self {
            Substance::Element(e, _) => &e.name,
            Substance::Compound(c) => &c.name,
        }
    }
    /// in kg/m3
    pub fn density(&self) -> f32 {
        match &self {
            Substance::Element(e, _) => e.density,
            Substance::Compound(c) => c.density,
        }
    }

    pub fn stopping_powers(&self, particle_type: ParticleType) -> Option<&StoppingPower> {
        match &self {
            Substance::Element(e, _) => e.stopping_powers.get(&particle_type),
            Substance::Compound(c) => c.stopping_powers.get(&particle_type),
        }
    }

    /// if all required info is available for it to absorb radiation
    pub fn is_absorber(&self) -> bool {
        match &self {
            Substance::Element(e, _) => e.is_absorber,
            Substance::Compound(c) => c.is_absorber,
        }
    }
}

impl PartialEq for Substance {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Element(arc, n) => match other {
                Self::Element(arc_other, n_other) => Arc::ptr_eq(arc, arc_other) && n == n_other,
                Self::Compound(_) => false,
            },
            Self::Compound(arc) => match other {
                Self::Element(_, _) => false,
                Self::Compound(arc_other) => Arc::ptr_eq(arc, arc_other),
            },
        }
    }
}

impl std::fmt::Display for Substance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Element(e, n) => {
                f.write_fmt(format_args!("{} {}", e.name, e.z + n))
            },
            Self::Compound(c) => {
                f.write_str(&c.name)
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
