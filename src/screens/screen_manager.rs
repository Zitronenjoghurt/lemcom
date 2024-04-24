use crate::screens::home::HomeScreen;
use std::collections::HashMap;

use super::{chat::ChatScreen, settings::SettingsScreen};

pub trait Screen {
    fn update(&mut self, ui: &mut egui::Ui) -> Option<ScreenId>;
}

#[derive(Eq, PartialEq, Hash)]
pub enum ScreenId {
    Chat,
    Home,
    Settings,
}

pub struct ScreenManager {
    screens: HashMap<ScreenId, Box<dyn Screen>>,
    current_screen_id: ScreenId,
}

impl ScreenManager {
    pub fn switch_screen(&mut self, screen_id: ScreenId) {
        if self.screens.contains_key(&screen_id) {
            self.current_screen_id = screen_id;
        }
    }

    pub fn update_current_screen(&mut self, ui: &mut egui::Ui) {
        if let Some(screen) = self.screens.get_mut(&self.current_screen_id) {
            if let Some(new_screen_id) = screen.update(ui) {
                self.switch_screen(new_screen_id);
            }
        }
    }
}

impl Default for ScreenManager {
    fn default() -> Self {
        let mut screens = HashMap::new();
        screens.insert(ScreenId::Home, Box::new(HomeScreen {}) as Box<dyn Screen>);
        screens.insert(ScreenId::Chat, Box::new(ChatScreen {}) as Box<dyn Screen>);
        screens.insert(
            ScreenId::Settings,
            Box::new(SettingsScreen {}) as Box<dyn Screen>,
        );

        Self {
            screens,
            current_screen_id: ScreenId::Home,
        }
    }
}
