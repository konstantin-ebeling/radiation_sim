use std::collections::{BTreeMap, HashMap};
use std::io::Cursor;

use serde::Deserialize;

use crate::{Constants, ParticleType};

/// (energy in eV, stopping power in eV/m | 1/m)
pub type StoppingPower = Vec<(f32, f32)>;

pub trait Substance {
    fn symbol(&self) -> &String;
    fn name(&self) -> &String;
    /// in kg/m3
    fn density(&self) -> f32;

    fn stopping_power(&self, particle_type: &ParticleType) -> &StoppingPower;

    /// if all required info is available for it to absorb radiation
    fn is_absorber(&self) -> bool;
}

#[derive(Debug)]
pub struct Element {
    pub z: usize,
    pub symbol: String,
    pub name: String,
    pub nucleon_ratio: f32,
    /// in eV
    pub energy: f32,
    /// in kg/m3
    pub density: f32,
    pub isotopes: BTreeMap<usize, Isotope>,
    pub stopping_powers: HashMap<ParticleType, StoppingPower>,

    pub is_absorber: bool,
}

#[derive(Debug)]
pub struct Isotope {
    pub z: usize,
    pub n: usize,
    /// in %
    pub abundance: f32,
    /// in s
    pub half_life: Option<f32>,
    /// in u
    pub atomic_mass: f32,
    pub decays: Vec<Decay>,
    /// in Bq/kg
    pub activity: Option<f32>,
}

#[derive(Debug)]
pub struct Decay {
    pub decay_type: DecayType,
    /// in eV
    pub decay_energy: f32,
    /// in eV
    pub gamma_energy: Option<f32>,
    pub is_usable: bool,
}

#[derive(Debug)]
pub enum DecayType {
    BetaMinus,
    BetaPlus,
    BetaElectronCapture,
    Alpha,
    Other,
}

impl Substance for Element {
    fn symbol(&self) -> &String {
        &self.symbol
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn density(&self) -> f32 {
        self.density
    }

    fn stopping_power(&self, particle_type: &ParticleType) -> &StoppingPower {
        &self.stopping_powers[particle_type]
    }

    fn is_absorber(&self) -> bool {
        self.is_absorber
    }
}

pub fn get_elemnts(constants: &Constants) -> Vec<Element> {
    let element_data = get_element_data();
    let isotope_data = get_isotope_data();

    let mut alpha_stopping_power = get_stopping_power(ParticleType::Alpha);
    let mut electron_stopping_power = get_stopping_power(ParticleType::Electron);
    let mut gamma_stopping_power = get_gamma_stopping_power();

    let activity_constant = constants.avogadro_constant * 2f32.log(std::f32::consts::E);

    element_data
        .into_iter()
        .map(|element| {
            let mut isotopes = BTreeMap::new();
            isotope_data
                .iter()
                .filter_map(|isotope| {
                    if isotope.z == element.z {
                        let half_life_raw = parse_num(isotope.half_life_sec.as_str());
                        let half_life = if half_life_raw == 0.0 {
                            None
                        } else {
                            Some(half_life_raw)
                        };

                        // convert micro u to u
                        let atomic_mass = parse_num(&isotope.atomic_mass) / 1_000_000.0;

                        let decay_energy = parse_num(isotope.decay_energy.as_str()) * 1_000_000.0;

                        // calculate Bq/g and conver to Bq/kg
                        let activity = half_life.clone().map(|half_life| {
                            (activity_constant / (half_life * atomic_mass)) * 1_000.0
                        });

                        Some(Isotope {
                            z: isotope.z,
                            n: isotope.n,
                            abundance: parse_num(isotope.abundance.as_str()),
                            half_life,

                            atomic_mass,
                            decays: vec![Decay {
                                decay_type: match isotope.decay_1.as_str() {
                                    "B-" => DecayType::BetaMinus,
                                    "B+" => DecayType::BetaPlus,
                                    "EC+B+" => DecayType::BetaElectronCapture,
                                    "A" => DecayType::Alpha,
                                    _ => DecayType::Other,
                                },
                                decay_energy,
                                gamma_energy: {
                                    let gamma_energy = parse_num(isotope.gamma_energy.as_str());
                                    if gamma_energy == 0.0 {
                                        None
                                    } else {
                                        Some(gamma_energy * 1_000_000.0)
                                    }
                                },
                                is_usable: decay_energy > 0.1,
                            }],
                            activity,
                        })
                    } else {
                        None
                    }
                })
                .for_each(|isotope| {
                    isotopes.insert(isotope.n, isotope);
                });

            // convert from g/cm3 to kg/m3
            let density = element.density * 1000.0;

            // stopping powers
            let mut stopping_powers = HashMap::new();

            let is_absorber = alpha_stopping_power.contains_key(&element.z)
                && electron_stopping_power.contains_key(&element.z)
                && gamma_stopping_power.contains_key(&element.z);

            // 1 cm2/g = 0.1 m2/kg
            // 1 MeV*cm2 = 100_000 eV*m2/kg
            // 1 eV*m2/kg * 1 kg/m3 = 1 eV/m
            if let Some(a) = alpha_stopping_power.remove(&element.z) {
                stopping_powers.insert(
                    ParticleType::Alpha,
                    a.into_iter()
                        .map(|(energy, stop_power)| {
                            (energy * 1_000_000.0, stop_power * 100_000.0 * density)
                        })
                        .collect(),
                );
            }
            if let Some(e) = electron_stopping_power.remove(&element.z) {
                stopping_powers.insert(
                    ParticleType::Electron,
                    e.into_iter()
                        .map(|(energy, stop_power)| {
                            (energy * 1_000_000.0, stop_power * 100_000.0 * density)
                        })
                        .collect(),
                );
            }
            // 1 cm2/g = 0.1 m2/kg
            // 1 m2/kg * 1 kg/m3 = 1/m
            if let Some(g) = gamma_stopping_power.remove(&element.z) {
                stopping_powers.insert(
                    ParticleType::Gamma,
                    g.into_iter()
                        .map(|(energy, stop_power)| {
                            (energy * 1_000_000.0, stop_power * 0.1 * density)
                        })
                        .collect(),
                );
            }

            Element {
                z: element.z,
                symbol: element.symbol,
                name: element.name,
                nucleon_ratio: element.nucleon_ratio,
                energy: element.energy,
                density,
                isotopes,
                stopping_powers,
                is_absorber,
            }
        })
        .collect()
}

#[derive(Debug, Deserialize)]
pub struct ElementDataRow {
    pub z: usize,
    pub symbol: String,
    pub name: String,
    pub nucleon_ratio: f32,
    /// in eV
    pub energy: f32,
    /// g/cm3
    pub density: f32,
}

pub fn get_element_data() -> Vec<ElementDataRow> {
    let mut data_reader = csv::Reader::from_reader(Cursor::new(include_bytes!(
        "./../assets/simulation_data/element_data.csv"
    )));
    data_reader
        .deserialize()
        .filter_map(|row| row.ok())
        .collect()
}

#[derive(Debug, Deserialize)]
pub struct IsotopeDataRow {
    pub z: usize,
    pub n: usize,
    pub radius: String,
    /// in %
    pub abundance: String,
    /// in s
    pub half_life_sec: String,
    pub decay_1: String,
    #[serde(rename = "decay_1_%")]
    pub decay_1_percent: String,
    pub decay_2: String,
    #[serde(rename = "decay_2_%")]
    pub decay_2_percent: String,
    pub decay_3: String,
    #[serde(rename = "decay_3_%")]
    pub decay_3_percent: String,
    /// in MeV
    pub decay_energy: String,
    /// in MeV
    pub gamma_energy: String,
    /// in micro u
    pub atomic_mass: String,
    pub massexcess: String,
}

pub fn get_isotope_data() -> Vec<IsotopeDataRow> {
    let mut data_reader = csv::Reader::from_reader(Cursor::new(include_bytes!(
        "./../assets/simulation_data/isotope_data.csv"
    )));
    data_reader
        .deserialize()
        .filter_map(|row| row.ok())
        .collect()
}

#[derive(Debug, Deserialize)]
pub struct StoppingPowerRow {
    /// in MeV
    pub energy: String,
    /// in MeV cm2/g
    pub stop_power: String,
}

// technically this is a mass attenuation coeffients but data reading and storing is similar
fn get_stopping_power(particle_type: ParticleType) -> HashMap<usize, Vec<(f32, f32)>> {
    #[rustfmt::skip]
    let table_data = match particle_type {
        ParticleType::Alpha => {vec![
            (1_usize, include_str!("./../assets/simulation_data/stopping_power_alpha/01.csv")),
            (2, include_str!("./../assets/simulation_data/stopping_power_alpha/02.csv")),
            (4, include_str!("./../assets/simulation_data/stopping_power_alpha/04.csv")),
            (6, include_str!("./../assets/simulation_data/stopping_power_alpha/06.csv")),
            (7, include_str!("./../assets/simulation_data/stopping_power_alpha/07.csv")),
            (8, include_str!("./../assets/simulation_data/stopping_power_alpha/08.csv")),
            (10, include_str!("./../assets/simulation_data/stopping_power_alpha/10.csv")),
            (13, include_str!("./../assets/simulation_data/stopping_power_alpha/13.csv")),
            (14, include_str!("./../assets/simulation_data/stopping_power_alpha/14.csv")),
            (18, include_str!("./../assets/simulation_data/stopping_power_alpha/18.csv")),
            (22, include_str!("./../assets/simulation_data/stopping_power_alpha/22.csv")),
            (26, include_str!("./../assets/simulation_data/stopping_power_alpha/26.csv")),
            (29, include_str!("./../assets/simulation_data/stopping_power_alpha/29.csv")),
            (32, include_str!("./../assets/simulation_data/stopping_power_alpha/32.csv")),
            (82, include_str!("./../assets/simulation_data/stopping_power_alpha/82.csv")),
        ]}
        ParticleType::Electron => {vec![
            (1_usize, include_str!("./../assets/simulation_data/stopping_power_electrons/01.csv")),
            (2, include_str!("./../assets/simulation_data/stopping_power_electrons/02.csv")),
            (3, include_str!("./../assets/simulation_data/stopping_power_electrons/03.csv")),
            (4, include_str!("./../assets/simulation_data/stopping_power_electrons/04.csv")),
            (5, include_str!("./../assets/simulation_data/stopping_power_electrons/05.csv")),
            (6, include_str!("./../assets/simulation_data/stopping_power_electrons/06.csv")),
            (7, include_str!("./../assets/simulation_data/stopping_power_electrons/07.csv")),
            (8, include_str!("./../assets/simulation_data/stopping_power_electrons/08.csv")),
            (9, include_str!("./../assets/simulation_data/stopping_power_electrons/09.csv")),
            (10, include_str!("./../assets/simulation_data/stopping_power_electrons/10.csv")),
            (11, include_str!("./../assets/simulation_data/stopping_power_electrons/11.csv")),
            (12, include_str!("./../assets/simulation_data/stopping_power_electrons/12.csv")),
            (13, include_str!("./../assets/simulation_data/stopping_power_electrons/13.csv")),
            (14, include_str!("./../assets/simulation_data/stopping_power_electrons/14.csv")),
            (82, include_str!("./../assets/simulation_data/stopping_power_electrons/82.csv")),
        ]}
        _ => panic!("requested stopping power table for not registered particle"),
    };

    let mut stopping_powers = HashMap::new();

    for (z, data) in table_data {
        let mut data_reader = csv::Reader::from_reader(Cursor::new(data));
        stopping_powers.insert(
            z,
            data_reader
                .deserialize()
                .filter_map(|row| row.ok())
                .map(|row: StoppingPowerRow| {
                    (
                        parse_num(row.energy.as_str()),
                        parse_num(row.stop_power.as_str()),
                    )
                })
                .collect(),
        );
    }

    stopping_powers
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

// technically this is a mass attenuation coeffients but data reading and storing is similar
fn get_gamma_stopping_power() -> HashMap<usize, Vec<(f32, f32)>> {
    #[rustfmt::skip]
    let table_data = vec![
        (1_usize, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/01.csv")),
        (2, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/02.csv")),
        (3, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/03.csv")),
        (4, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/04.csv")),
        (5, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/05.csv")),
        (6, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/06.csv")),
        (7, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/07.csv")),
        (8, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/08.csv")),
        (9, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/09.csv")),
        (10, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/10.csv")),
        (11, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/11.csv")),
        (12, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/12.csv")),
        (13, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/13.csv")),
        (14, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/14.csv")),
        (82, include_str!("./../assets/simulation_data/mass_attenuation_coefficients/82.csv")),
    ];

    let mut stopping_powers = HashMap::new();

    for (z, data) in table_data {
        let mut data_reader = csv::Reader::from_reader(Cursor::new(data));
        stopping_powers.insert(
            z,
            data_reader
                .deserialize()
                .filter_map(|row| row.ok())
                .map(|row: MassAttenuationCoefficientRow| {
                    (parse_num(row.energy.as_str()), parse_num(row.yp.as_str()))
                })
                .collect(),
        );
    }

    stopping_powers
}


#[derive(Debug)]
pub struct Compound {
    pub symbol: String,
    pub name: String,
    /// in kg/m3
    pub density: f32,
    pub stopping_powers: HashMap<ParticleType, StoppingPower>,

    pub is_absorber: bool,
}

impl Substance for Compound {
    fn symbol(&self) -> &String {
        &self.symbol
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn density(&self) -> f32 {
        self.density
    }

    fn stopping_power(&self, particle_type: &ParticleType) -> &StoppingPower {
        &self.stopping_powers[particle_type]
    }

    fn is_absorber(&self) -> bool {
        self.is_absorber
    }
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
