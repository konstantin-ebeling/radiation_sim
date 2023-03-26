use lazy_static::lazy_static;

pub const LIGHT_SPEED: f64 = 299_792_458.0;
pub const LIGHT_SPEED_SQ: f64 = 89_875_517_873_681_764.0;
lazy_static! {
    pub static ref AVOGADRO_CONSTANT: f64 = 6.022_141 * (10f64).powi(23);
    pub static ref EV_CONVERSION: f64 = 1.602 * (10f64).powi(-19);
    pub static ref ELECTRON_MASS: f64 = 9.109_384 * (10f64).powi(-31);
    pub static ref ALPHA_MASS: f64 = 6.644_657 * (10f64).powi(-27);
}
