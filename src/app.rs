use crate::{screens::screen_manager::ScreenManager, store::Store};
use egui::mutex::RwLock;

#[derive(Default)]
pub struct LemComApp {
    screen_manager: ScreenManager,
    store: RwLock<Store>,
}

impl LemComApp {
    pub fn new() -> Self {
        let store = Store::load().unwrap_or_else(|_| Store::default());

        LemComApp {
            screen_manager: ScreenManager::default(),
            store: RwLock::new(store),
        }
    }
}

impl eframe::App for LemComApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.screen_manager.update_current_screen(ui, &self.store);
        });
    }
}
