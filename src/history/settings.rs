use super::History;
use arc_util::settings::HasSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistorySettings {
    pub max_fights: usize,
    pub min_duration: u64,
}

impl HistorySettings {
    pub const fn new(max_fights: usize, min_duration: u64) -> Self {
        Self {
            max_fights,
            min_duration,
        }
    }
}

impl<T> HasSettings for History<T> {
    type Settings = HistorySettings;

    const SETTINGS_ID: &'static str = "history";

    fn current_settings(&self) -> Self::Settings {
        self.settings.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        self.settings = loaded;
    }
}
