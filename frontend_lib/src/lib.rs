//! Demo app for egui
#![allow(clippy::missing_errors_doc)]

pub mod ui_model;
mod widgets;

use crate::ui_model::PortalState;
use crate::widgets::ui_extensions::UiExtension;
use eframe::egui;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use eframe::web::AppRunnerRef;

/// Time of day as seconds since midnight. Used for clock in demo app.
// pub(crate) fn seconds_since_midnight() -> f64 {
//     use chrono::Timelike;
//     let time = chrono::Local::now().time();
//     time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
// }

// ----------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebHandle {
    handle: AppRunnerRef,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebHandle {
    #[wasm_bindgen]
    pub fn stop_web(&self) -> Result<(), wasm_bindgen::JsValue> {
        let mut app = self.handle.lock();
        app.destroy()
    }

    // #[wasm_bindgen]
    // pub fn set_some_content_from_javasript(&mut self, _some_data: &str) {
    //     let _app = self.handle.lock().app_mut::<WrapApp>();
    //     // _app.data = some_data;
    // }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_wasm_hooks() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start_separate(canvas_id: &str) -> Result<WebHandle, wasm_bindgen::JsValue> {
    let web_options = eframe::WebOptions::default();
    // let options = eframe::WebOptions::default();
    // let app_name = "MyApp";
    // eframe::start_web(app_name, options, Box::new(|_cc| Box::<PortalState>::default()));
    eframe::start_web(
        canvas_id,
        web_options,
        Box::new(|_| Box::new(PortalState::default())),
    )
    .await
    .map(|handle| WebHandle { handle })
}

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start(canvas_id: &str) -> Result<WebHandle, wasm_bindgen::JsValue> {
    init_wasm_hooks();
    start_separate(canvas_id).await
}

impl eframe::App for PortalState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_height = ui.available_height();
            ui.horizontal(|ui| {
                ui.menu(available_height);

                ui.separator();

                ui.chat_group_table(self);

                ui.separator();

                ui.chat_table(self)
            })
        });
    }
}
