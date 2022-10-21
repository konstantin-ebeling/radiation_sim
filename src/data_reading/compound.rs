use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

use serde::Deserialize;

use crate::{ParticleType, StoppingPower};

use super::{parse_num, MassAttenuationCoefficientRow, StoppingPowerRow};

#[derive(Debug, Clone)]
pub struct Compound {
    pub symbol: String,
    pub name: String,
    /// in kg/m3
    pub density: f32,
    pub stopping_powers: HashMap<ParticleType, StoppingPower>,

    pub is_absorber: bool,
}

pub fn get_compounds() -> Vec<Arc<Compound>> {
    let compound_data = get_compound_data();

    let mut alpha_stopping_power = get_stopping_power(ParticleType::Alpha);
    let mut electron_stopping_power = get_stopping_power(ParticleType::Electron);
    let mut gamma_stopping_power = get_gamma_stopping_power();

    compound_data
        .into_iter()
        .map(|compound| {
            // convert from g/cm3 to kg/m3
            let density = compound.density * 1000.0;

            // stopping powers
            let mut stopping_powers = HashMap::new();

            let is_absorber = alpha_stopping_power.contains_key(&compound.name)
                && electron_stopping_power.contains_key(&compound.name)
                && gamma_stopping_power.contains_key(&compound.name);

            // 1 cm2/g = 0.1 m2/kg =>
            // 1 MeV*cm2/g = 100_000 eV*m2/kg
            // 1 eV*m2/kg * 1 kg/m3 = 1 eV/m
            if let Some(a) = alpha_stopping_power.remove(&compound.name) {
                stopping_powers.insert(
                    ParticleType::Alpha,
                    a.into_iter()
                        .map(|(energy, stop_power)| {
                            (energy * 1_000_000.0, stop_power * 100_000.0 * density)
                        })
                        .collect(),
                );
            }
            if let Some(e) = electron_stopping_power.remove(&compound.name) {
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
            if let Some(g) = gamma_stopping_power.remove(&compound.name) {
                stopping_powers.insert(
                    ParticleType::Gamma,
                    g.into_iter()
                        .map(|(energy, stop_power)| {
                            (energy * 1_000_000.0, stop_power * 0.1 * density)
                        })
                        .collect(),
                );
            }

            Arc::new(Compound {
                symbol: compound.symbol,
                name: compound.name,
                density,
                stopping_powers,
                is_absorber,
            })
        })
        .collect()
}

#[derive(Debug, Deserialize)]
pub struct CompoundDataRow {
    pub symbol: String,
    pub name: String,
    pub nucleon_ratio: f32,
    /// in eV
    pub energy: f32,
    /// g/cm3
    pub density: f32,
}

fn get_compound_data() -> Vec<CompoundDataRow> {
    let mut data_reader = csv::Reader::from_reader(Cursor::new(include_bytes!(
        "./../../assets/simulation_data/compound_data.csv"
    )));
    data_reader
        .deserialize()
        .filter_map(|row| row.ok())
        .collect()
}

// technically this is a mass attenuation coeffients but data reading and storing is similar
fn get_stopping_power(particle_type: ParticleType) -> HashMap<String, Vec<(f32, f32)>> {
    #[rustfmt::skip]
    let table_data = match particle_type {
        ParticleType::Alpha => {vec![
            ("Air", include_str!("./../../assets/simulation_data/stopping_power_alpha/Air.csv")),
            ("Water", include_str!("./../../assets/simulation_data/stopping_power_alpha/Water.csv")),
            ("Vacuum", include_str!("./../../assets/simulation_data/stopping_power_alpha/Vacuum.csv")),
        ]}
        ParticleType::Electron => {vec![
            ("Air", include_str!("./../../assets/simulation_data/stopping_power_electrons/Air.csv")),
            ("Water", include_str!("./../../assets/simulation_data/stopping_power_electrons/Water.csv")),
            ("Vacuum", include_str!("./../../assets/simulation_data/stopping_power_electrons/Vacuum.csv")),
        ]}
        _ => panic!("requested stopping power table for not registered particle"),
    };

    let mut stopping_powers = HashMap::new();

    for (name, data) in table_data {
        let mut data_reader = csv::Reader::from_reader(Cursor::new(data));
        stopping_powers.insert(
            name.to_owned(),
            data_reader
                .deserialize()
                .filter_map(|row| {
                    row.map_err(|e| {
                        log::warn!("Error reading row ({}, a/e): {}", &name, e);
                        e
                    })
                    .ok()
                })
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

// technically this is a mass attenuation coeffients but data reading and storing is similar
fn get_gamma_stopping_power() -> HashMap<String, Vec<(f32, f32)>> {
    #[rustfmt::skip]
    let table_data = vec![
        ("Air", include_str!("./../../assets/simulation_data/mass_attenuation_coefficients/Air.csv")),
        ("Water", include_str!("./../../assets/simulation_data/mass_attenuation_coefficients/Water.csv")),
        ("Vacuum", include_str!("./../../assets/simulation_data/mass_attenuation_coefficients/Vacuum.csv")),
    ];

    let mut stopping_powers = HashMap::new();

    for (name, data) in table_data {
        let mut data_reader = csv::Reader::from_reader(Cursor::new(data));
        stopping_powers.insert(
            name.to_owned(),
            data_reader
                .deserialize()
                .filter_map(|row| {
                    row.map_err(|e| {
                        log::warn!("Error reading row ({}, g): {}", &name, e);
                        e
                    })
                    .ok()
                })
                .map(|row: MassAttenuationCoefficientRow| {
                    (parse_num(row.energy.as_str()), parse_num(row.yp.as_str()))
                })
                .collect(),
        );
    }

    stopping_powers
}
