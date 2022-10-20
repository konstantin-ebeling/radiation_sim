use crate::{data_reading::Substance, Constants};

use super::SubstanceIdentifier;

pub mod presets;

#[derive(Debug, Default, Clone)]
pub struct MaterialData {
    pub parts: Vec<(f32, SubstanceIdentifier)>,
}

impl MaterialData {
    pub fn pick_substance(&self, constants: &Constants) -> Substance {
        let num = fastrand::f32();
        let mut acc = 0.0;
        let mut i = 0;

        loop {
            if self.parts[i].0 + acc > num {
                break self.parts[i].1.get(&constants);
            }

            acc += self.parts[i].0;
            i += 1;
        }
    }
}
