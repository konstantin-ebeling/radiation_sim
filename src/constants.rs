use lazy_static::lazy_static;

pub const LIGHT_SPEED: f32 = 299_792_458.0;
pub const LIGHT_SPEED_SQ: f32 = 89_875_517_873_681_764.0;
lazy_static! {
    pub static ref AVOGADRO_CONSTANT: f32 = 6.022_141 * (10f32).powi(23);
    pub static ref EV_CONVERSION: f32 = 1.602 * (10f32).powi(-19);
    pub static ref ELECTRON_MASS: f32 = 9.109_384 * (10f32).powi(-31);
    pub static ref ALPHA_MASS: f32 = 6.644_657 * (10f32).powi(-27);
}
