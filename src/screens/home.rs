use crate::screens::screen_manager::Screen;

pub struct HomeScreen {}

impl Screen for HomeScreen {
    fn update(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);

            ui.heading("LemCom Messenger");
            ui.add_space(10.0);

            if ui.button("Button 1").clicked() {
                println!("Button 1 clicked");
            }
            if ui.button("Button 2").clicked() {
                println!("Button 2 clicked");
            }
        });
    }
}
