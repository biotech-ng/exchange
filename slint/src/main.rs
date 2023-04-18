use slint_lib::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Preview: Style
// The style to be used for the preview (eg: 'fluent', 'material', or 'native')

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let window = PortalApp::new().unwrap();

    window.run().unwrap()
}
