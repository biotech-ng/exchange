slint::include_modules!();

use std::rc::Rc;
use slint::VecModel;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct PortalState {
    chats: Rc<VecModel<ChatRowData>>
}

impl Default for PortalState {
    fn default() -> Self {
        let items: Vec<ChatRowData> = (0..20).map(|i| {
            ChatRowData {
                chat_name: format!("{} Some Chat Some Chat Some Chat Some Chat Some Chat Some Chat Some Chat Some Chat", i).into(),
                last_message_time_or_date: format!("00:0{}", i).into(),
            }
        }).collect();

        let chats = Rc::new(slint::VecModel::from(items));

        PortalState { chats }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn start_ui() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let window = PortalApp::new().unwrap();

    let state = PortalState::default();

    window.set_memory_chats(state.chats.into());

    window.run().unwrap()
}
