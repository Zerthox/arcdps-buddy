pub mod event;
pub mod ui;

use crate::{
    combat::FightData,
    data::{LoadError, SkillData},
    history::History,
    ui::{boon_log::BoonLog, cast_log::CastLog, history::HistoryView, multi_view::MultiView},
};
use arc_util::{
    settings::Settings,
    ui::{Window, WindowOptions},
};
use log::{info, warn};
use once_cell::sync::Lazy;
use semver::Version;
use std::sync::{Mutex, MutexGuard};

/// Plugin version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Settings file name.
const SETTINGS_FILE: &str = "arcdps_buddy.json";

/// Cast skill definition file name.
const SKILLS_FILE: &str = "arcdps_buddy_skills.yml";

/// Main plugin instance.
// FIXME: a single mutex for the whole thing is potentially inefficient
static PLUGIN: Lazy<Mutex<Plugin>> = Lazy::new(|| Mutex::new(Plugin::new()));

/// Main plugin.
#[derive(Debug)]
pub struct Plugin {
    data: SkillData,
    data_state: Result<(), LoadError>,

    start: Option<u64>,
    history: History<FightData>,

    view: HistoryView,
    multi_view: Window<MultiView>,
    cast_log: Window<CastLog>,
    boon_log: Window<BoonLog>,
}

impl Plugin {
    /// Creates a new plugin.
    pub fn new() -> Self {
        Self {
            data: SkillData::with_defaults(),
            data_state: Err(LoadError::NotFound),

            start: None,
            history: History::new(10),

            view: HistoryView::new(),
            multi_view: Window::with_default(
                "Buddy Multi",
                WindowOptions {
                    width: 350.0,
                    height: 450.0,
                    ..Default::default()
                },
            ),
            cast_log: Window::with_default(
                "Buddy Casts",
                WindowOptions {
                    width: 350.0,
                    height: 450.0,
                    ..Default::default()
                },
            ),
            boon_log: Window::with_default(
                "Buddy Boons",
                WindowOptions {
                    width: 350.0,
                    height: 450.0,
                    ..Default::default()
                },
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
        settings.load_component(&mut self.multi_view);
        settings.load_component(&mut self.cast_log);
        settings.load_component(&mut self.boon_log);

        self.load_data();
    }

    pub fn load_data(&mut self) {
        if let Some(path) = Settings::config_path(SKILLS_FILE) {
            if path.exists() {
                self.data_state = self.data.try_load(&path);

                if self.data_state.is_ok() {
                    info!("Loaded custom definitions from \"{}\"", path.display());
                } else {
                    warn!(
                        "Failed to load custom definitions from \"{}\"",
                        path.display()
                    );
                }
            }
        }
    }

    pub fn reset_data(&mut self) {
        self.data = SkillData::with_defaults();
        self.data_state = Err(LoadError::NotFound);
    }

    /// Unloads the plugin.
    pub fn unload(&mut self) {
        let mut settings = Settings::from_file(SETTINGS_FILE);
        settings.store_data("version", VERSION);
        settings.store_component(&self.multi_view);
        settings.store_component(&self.cast_log);
        settings.store_component(&self.boon_log);
        settings.save_file();
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}
