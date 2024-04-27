use super::screen_manager::ScreenId;
use crate::{screens::screen_manager::Screen, store::Store};
use egui::mutex::RwLock;

pub struct HomeScreen {}

impl Screen for HomeScreen {
    fn update(&mut self, ui: &mut egui::Ui, store: &RwLock<Store>) -> Option<ScreenId> {
        let mut style = (*ui.ctx().style()).clone();
        let mut redirect_screen: Option<ScreenId> = None;

        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(24.0, eframe::epaint::FontFamily::Proportional),
        );

        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(80.0, eframe::epaint::FontFamily::Proportional),
        );

        ui.ctx().set_style(style);

        ui.vertical_centered(|ui| {
            ui.add_space(50.0);

            ui.heading("LemCom Messenger");
            ui.add_space(20.0);

            if ui
                .add_sized([120., 40.], egui::Button::new("Chat"))
                .clicked()
            {
                redirect_screen = Some(ScreenId::Chat);
            }

            ui.add_space(10.0);

            if ui
                .add_sized([120., 40.], egui::Button::new("Settings"))
                .clicked()
            {
                redirect_screen = Some(ScreenId::Settings);
            }
        });

        let store_read = store.read();
        let mut show_api_key_popup = store_read.api_key_available();
        let mut api_key_input: String = String::new();

        if show_api_key_popup {
            egui::Window::new("API Key not set")
                .open(&mut show_api_key_popup)
                .show(ctx, |ui| {
                    ui.label("Enter your API Key:");
                    ui.text_edit_singleline(&mut api_key_input);
                    if ui.button("Submit").clicked() {
                        show_api_key_popup = false;
                    }
                });
        }

        redirect_screen
    }
}
