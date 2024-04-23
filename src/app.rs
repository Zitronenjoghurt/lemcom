use crate::screens::screen_manager::ScreenManager;

#[derive(Default)]
pub struct LemComApp {
    screen_manager: ScreenManager,
}

impl eframe::App for LemComApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.screen_manager.update_current_screen(ui);
        });
    }
}
