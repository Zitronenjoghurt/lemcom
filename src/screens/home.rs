use crate::screens::screen_manager::Screen;

use super::screen_manager::ScreenId;

pub struct HomeScreen {}

impl Screen for HomeScreen {
    fn update(&mut self, ui: &mut egui::Ui) -> Option<ScreenId> {
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

        redirect_screen
    }
}
