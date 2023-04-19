slint::include_modules!();

use std::rc::Rc;
use slint::VecModel;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

struct PortalState {
    chats: Rc<VecModel<ChatRowData>>,
    messages: Rc<VecModel<ChatMessageData>>,
}

impl Default for PortalState {
    fn default() -> Self {
        let items: Vec<_> = (0..20).map(|i| {
            ChatRowData {
                chat_name: format!("{} Some Chat Some Chat Some Chat Some Chat Some Chat Some Chat Some Chat Some Chat", i).into(),
                last_message_time_or_date: format!("00:0{}", i).into(),
                last_message: "Some Chat Some Chat Some Chat Some Chat Some Chat Some Chat Some Chat Some Chat".into(),
                unread_messages: 90 + i
            }
        }).collect();

        let chats = Rc::new(slint::VecModel::from(items));

        let messages: Vec<_> = vec![0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 2].into_iter().map(|i| {
            ChatMessageData {
                message_type: i,
                message: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".into(),
                plugin_picture: "assets/plugin.png".into(),
            }
        }).collect();

        let messages = Rc::new(slint::VecModel::from(messages));

        PortalState { chats, messages }
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
    window.set_memory_messages(state.messages.into());

    window.run().unwrap()
}
