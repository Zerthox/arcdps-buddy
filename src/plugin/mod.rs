pub mod event;
pub mod ui;

use crate::{cast::Cast, cast_log::CastLog, data::SkillData};
use arc_util::{
    settings::Settings,
    ui::{Window, WindowOptions},
};
use log::info;
use once_cell::sync::Lazy;
use semver::Version;
use std::sync::{Mutex, MutexGuard};

/// Plugin version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Settings file name.
const SETTINGS_FILE: &str = "arcdps_buddy.json";

/// Main plugin instance.
// FIXME: a single mutex for the whole thing is potentially inefficient
static PLUGIN: Lazy<Mutex<Plugin>> = Lazy::new(|| Mutex::new(Plugin::new()));

/// Main plugin.
#[derive(Debug)]
pub struct Plugin {
    data: SkillData,
    start: Option<u64>,
    casts: Vec<Cast>,
    cast_log: Window<CastLog>,
}

impl Plugin {
    /// Creates a new plugin.
    pub fn new() -> Self {
        Self {
            data: SkillData::with_defaults(),
            start: None,
            casts: Vec::new(),
            cast_log: Window::new(
                WindowOptions {
                    width: 350.0,
                    height: 450.0,
                    ..WindowOptions::new("Casts")
                },
                CastLog::new(),
            ),
        }
    }

    /// Acquires access to the plugin instance.
    pub fn lock() -> MutexGuard<'static, Self> {
        PLUGIN.lock().unwrap()
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
