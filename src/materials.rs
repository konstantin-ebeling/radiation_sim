use std::collections::HashMap;

use super::{ParticleMaterial, ParticleType};

pub fn vacuum_material() -> ParticleMaterial {
    let mut material = HashMap::new();
    material.insert(ParticleType::Alpha, 0.0);
    material.insert(ParticleType::Electron, 0.0);
    material.insert(ParticleType::Gamma, 0.0);
    material.insert(ParticleType::Neutron, 0.0);
    material.insert(ParticleType::Proton, 0.0);
    material
}

pub fn air_material() -> ParticleMaterial {
    let mut material = HashMap::new();
    material.insert(ParticleType::Alpha, 0.001);
    material.insert(ParticleType::Electron, 0.0001);
    material.insert(ParticleType::Gamma, 0.00005);
    material.insert(ParticleType::Neutron, 0.0);
    material.insert(ParticleType::Proton, 0.0);
    material
}

pub fn dense_material() -> ParticleMaterial {
    let mut material = HashMap::new();
    material.insert(ParticleType::Alpha, 1000.0);
    material.insert(ParticleType::Electron, 10000.0);
    material.insert(ParticleType::Gamma, 1000.0);
    material.insert(ParticleType::Neutron, -0.1);
    material.insert(ParticleType::Proton, -0.1);
    material
}
