pub mod app;
pub mod screens {
    pub mod home;
    pub mod screen_manager;
}

fn main() {
    let _ = eframe::run_native(
        "LemCom Messenger",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::<app::LemComApp>::default()),
    );
}
