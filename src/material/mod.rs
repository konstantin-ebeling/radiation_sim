use crate::data_reading::Substance;

pub mod presets;

#[derive(Debug, Default, Clone)]
pub struct MaterialData {
    pub parts: Vec<(f32, Substance)>,
}

impl MaterialData {
    pub fn pick_substance(&self) -> Substance {
        let num = fastrand::f32();
        let mut acc = 0.0;
        let mut i = 0;

        loop {
            if self.parts[i].0 + acc > num {
                break self.parts[i].1.clone();
            }

            acc += self.parts[i].0;
            i += 1;
        }
    }
}
