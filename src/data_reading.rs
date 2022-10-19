use std::collections::BTreeMap;
use std::io::Cursor;

use serde::Deserialize;

pub struct Element {
    pub z: usize,
    pub symbol: String,
    pub name: String,
    pub nucleon_ratio: f32,
    pub energy: f32,
    pub density: f32,
    pub isotopes: BTreeMap<usize, Isotope>,
}

#[derive(Debug)]
pub struct Isotope {
    pub z: usize,
    pub n: usize,
    pub abundance: f32,
    pub half_life: Option<f32>,
    pub atomic_mass: f32,
    pub decays: Vec<Decay>,
}

#[derive(Debug)]
pub struct Decay {
    pub decay_type: DecayType,
    pub decay_energy: f32,
    pub gamma_energy: Option<f32>,
}

#[derive(Debug)]
pub enum DecayType {
    BetaMinus,
    BetaPlus,
    BetaElectronCapture,
    Alpha,
    Other,
}

pub fn get_elemnts() -> Vec<Element> {
    let element_data = get_element_data();
    let isotope_data = get_isotope_data();

    element_data
        .into_iter()
        .map(|element| {
            let mut isotopes = BTreeMap::new();
            isotope_data
                .iter()
                .filter_map(|isotope| {
                    if isotope.z == element.z {
                        Some(Isotope {
                            z: isotope.z,
                            n: isotope.n,
                            abundance: parse_num(isotope.abundance.as_str()),
                            half_life: {
                                let half_life = parse_num(isotope.half_life_sec.as_str());
                                if half_life == 0.0 {
                                    None
                                } else {
                                    Some(half_life)
                                }
                            },
                            atomic_mass: parse_num(&isotope.atomic_mass),
                            decays: vec![Decay {
                                decay_type: match isotope.decay_1.as_str() {
                                    "B-" => DecayType::BetaMinus,
                                    "B+" => DecayType::BetaPlus,
                                    "EC+B+" => DecayType::BetaElectronCapture,
                                    "A" => DecayType::Alpha,
                                    _ => DecayType::Other,
                                },
                                decay_energy: parse_num(isotope.decay_energy.as_str()),
                                gamma_energy: {
                                    let gamma_energy = parse_num(isotope.gamma_energy.as_str());
                                    if gamma_energy == 0.0 {
                                        None
                                    } else {
                                        Some(gamma_energy)
                                    }
                                },
                            }],
                        })
                    } else {
                        None
                    }
                })
                .for_each(|isotope| {
                    isotopes.insert(isotope.n, isotope);
                });

            Element {
                z: element.z,
                symbol: element.symbol,
                name: element.name,
                nucleon_ratio: element.nucleon_ratio,
                energy: element.energy,
                // convert from g/cm3 to kg/m3
                density: element.density * 1000.0,
                isotopes: isotopes,
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
    pub energy: f32,
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
    pub abundance: String,
    pub half_life_sec: String,
    pub decay_1: String,
    #[serde(rename = "decay_1_%")]
    pub decay_1_percent: String,
    pub decay_2: String,
    #[serde(rename = "decay_2_%")]
    pub decay_2_percent: String,
    pub decay_3: String,
    #[serde(rename = "decay_2_%")]
    pub decay_3_percent: String,
    pub decay_energy: String,
    pub gamma_energy: String,
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
