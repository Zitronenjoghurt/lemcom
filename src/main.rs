use app::LemComApp;

pub mod app;
pub mod screens {
    pub mod chat;
    pub mod home;
    pub mod screen_manager;
    pub mod settings;
}
pub mod store;

fn main() {
    let _ = eframe::run_native(
        "LemCom Messenger",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(LemComApp::new())),
    );
}
