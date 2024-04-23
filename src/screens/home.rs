use crate::screens::screen_manager::Screen;

pub struct HomeScreen {
    pub value: u32,
}

impl Screen for HomeScreen {
    fn update(&mut self, ui: &mut egui::Ui) {
        ui.label("Home Screen");
        ui.add(egui::Slider::new(&mut self.value, 1..=120).text("Slider"));
    }
}
