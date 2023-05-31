pub mod event;
pub mod ui;

use crate::{cast::Cast, cast_log::CastLog, data::Data};
use arc_util::{
    settings::Settings,
    ui::{Window, WindowOptions},
};
use log::info;
use semver::Version;

/// Plugin version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Settings file name.
const SETTINGS_FILE: &str = "arcdps_buddy.json";

/// Main plugin.
#[derive(Debug)]
pub struct Plugin {
    data: Data,
    start: Option<u64>,
    casts: Vec<Cast>,
    cast_log: Window<CastLog>,
}

impl Plugin {
    /// Creates a new plugin.
    pub fn new() -> Self {
        Self {
            data: Data::with_defaults(),
            start: None,
            casts: Vec::new(),
            cast_log: Window::new(WindowOptions::new("Casts"), CastLog::new()),
        }
    }

    /// Loads the plugin.
    pub fn load(&mut self) {
        info!("v{} load", VERSION);

        // load settings
        let mut settings = Settings::from_file(SETTINGS_FILE);
        let settings_version: Option<Version> = settings.load_data("version");
        info!(
            "Loaded settings from version {}",
            match &settings_version {
                Some(version) => version.to_string(),
                None => "unknown".into(),
            }
        );
        settings.load_component(&mut self.cast_log);
    }

    /// Unloads the plugin.
    pub fn unload(&mut self) {
        let mut settings = Settings::from_file(SETTINGS_FILE);
        settings.store_data("version", VERSION);
        settings.store_component(&self.cast_log);
        settings.save_file();
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}
