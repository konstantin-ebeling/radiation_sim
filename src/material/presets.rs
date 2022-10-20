use crate::{Substance, SubstanceData};

use super::MaterialData;

pub fn h3(data: &SubstanceData) -> MaterialData {
    MaterialData {
        parts: vec![(1.0, Substance::Element(data.elements[&1].clone(), 2))],
    }
}

pub fn pb208(data: &SubstanceData) -> MaterialData {
    MaterialData {
        parts: vec![(1.0, Substance::Element(data.elements[&82].clone(), 126))],
    }
}

pub fn pb210(data: &SubstanceData) -> MaterialData {
    MaterialData {
        parts: vec![(1.0, Substance::Element(data.elements[&82].clone(), 128))],
    }
}

pub fn pu239(data: &SubstanceData) -> MaterialData {
    MaterialData {
        parts: vec![(1.0, Substance::Element(data.elements[&94].clone(), 145))],
    }
}

pub fn air(data: &SubstanceData) -> MaterialData {
    dbg!(&data.compounds);
    MaterialData {
        parts: vec![(1.0, Substance::Compound(data.compounds[&"Air".to_owned()].clone()))],
    }
}
