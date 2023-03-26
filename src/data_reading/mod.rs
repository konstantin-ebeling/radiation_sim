use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use bevy::prelude::*;
use serde::Deserialize;

use crate::ParticleType;

pub mod compound;
pub mod element;

pub use compound::Compound;
pub use element::Element;

/// (energy in eV, stopping power in eV/m | 1/m)
pub type StoppingPower = Vec<(f32, f32)>;

#[derive(Debug, Clone, Reflect, FromReflect)]
pub enum Substance {
    Element(#[reflect(ignore)] Arc<Element>, usize),
    Compound(#[reflect(ignore)] Arc<Compound>),
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
            Self::Element(e, n) => f.write_fmt(format_args!("{} {}", e.name, e.z + n)),
            Self::Compound(c) => f.write_str(&c.name),
        }
    }
}

impl Default for Substance {
    fn default() -> Self {
        lazy_static::lazy_static! {
            static ref VACUUM: Arc<Compound> = {
                let stopping_powers = HashMap::from([
                    (ParticleType::Alpha, vec![(0.0, 0.0)]),
                    (ParticleType::Electron, vec![(0.0, 0.0)]),
                    (ParticleType::Gamma, vec![(0.0, 0.0)]),
                ]);
                Arc::new(Compound {
                    symbol: "Vac".to_owned(),
                    name: "Vakuum".to_owned(),
                    density: 0.0,
                    stopping_powers,
                    is_absorber: true,
                })
            };
        }

        Self::Compound(VACUUM.clone())
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
    if !num.contains('E') {
        num.parse().unwrap_or(0.0)
    } else {
        let num = num.split_once('E').unwrap();
        num.0.parse().unwrap_or(0.0) * 10_f32.powi(num.1.parse().unwrap_or(0))
    }
}

pub struct RadiationSimData;

impl Plugin for RadiationSimData {
    fn build(&self, app: &mut App) {
        app.insert_resource(SubstanceData {
            elements: BTreeMap::new(),
            compounds: BTreeMap::new(),
            radiators: Vec::new(),
            absorbers: Vec::new(),
        })
        .add_startup_system(read_data.in_base_set(StartupSet::PreStartup));
    }
}

#[derive(Debug, Resource)]
pub struct SubstanceData {
    pub elements: BTreeMap<usize, Arc<Element>>,
    pub compounds: BTreeMap<String, Arc<Compound>>,
    pub radiators: Vec<Substance>,
    pub absorbers: Vec<Substance>,
}

pub fn read_data(mut substance_data: ResMut<SubstanceData>) {
    // elements
    let element_data = element::get_elements();
    let mut element_btree = BTreeMap::new();
    for element in element_data {
        element_btree.insert(element.z, element);
    }
    substance_data.elements = element_btree;

    let mut radiators = Vec::new();
    let mut absorbers = Vec::new();
    for (_, element) in &substance_data.elements {
        for (n, isotope) in &element.isotopes {
            if isotope.is_usable {
                radiators.push(Substance::Element(element.clone(), *n));
            }
        }
        if element.is_absorber {
            let mut isotopes_sorted = element.isotopes.clone().into_values().collect::<Vec<_>>();
            isotopes_sorted.sort_by_key(|i| i.abundance);
            absorbers.push(Substance::Element(
                element.clone(),
                isotopes_sorted.last().unwrap().n,
            ));
        }
    }
    substance_data.radiators = radiators;
    substance_data.absorbers = absorbers;

    // compounds
    let compound_data = compound::get_compounds();
    let mut compound_btree = BTreeMap::new();
    for compound in compound_data {
        compound_btree.insert(compound.name.to_owned(), compound);
    }
    substance_data.compounds = compound_btree;

    let mut absorbers = Vec::new();
    for (_, compound) in &substance_data.compounds {
        if compound.is_absorber {
            absorbers.push(Substance::Compound(compound.clone()));
        }
    }
    substance_data.absorbers.extend(absorbers);

    // nice logs
    for e in &substance_data.radiators {
        match &e {
            Substance::Element(element, n) => {
                let isotope = &element.isotopes[n];
                log::info!(
                    "{} {:?}: {:?} eV, {:?} ev, {} Bq/kg",
                    element.symbol,
                    element.z + n,
                    isotope.decays[0].decay_energy,
                    isotope.decays[0].gamma_energy,
                    isotope.activity.unwrap()
                );
            }
            Substance::Compound(compound) => {
                log::info!("{}", &compound.name);
            }
        }
    }

    for e in &substance_data.absorbers {
        match &e {
            Substance::Element(element, _) => {
                log::info!("{} Absorber", element.symbol);
            }
            Substance::Compound(compound) => {
                log::info!("{} Absorber", compound.name);
            }
        }
    }
}
