use crate::SubstanceIdentifier;

use super::MaterialData;

pub fn lead() -> MaterialData {
    MaterialData {
        parts: vec![(1.0, SubstanceIdentifier::Element(82, 126))],
    }
}

pub fn air() -> MaterialData {
    MaterialData {
        parts: vec![(1.0, SubstanceIdentifier::Compound("Air".to_owned()))],
    }
}

pub fn plutonium() -> MaterialData {
    MaterialData {
        parts: vec![(1.0, SubstanceIdentifier::Element(94, 145))],
    }
}
