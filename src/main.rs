use radiation_sim::run;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    run();
}

// rustc also needs this in wasm to be happy
fn main() {
    run();
}
