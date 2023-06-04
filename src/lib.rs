mod cast;
mod data;
mod plugin;
mod skill;
mod ui;

use arcdps::{imgui::Ui, Agent, CombatEvent};
use plugin::Plugin;

// create exports for arcdps
arcdps::export! {
    name: "Buddy",
    sig: 0x84c13713,
    init,
    release,
    combat,
    imgui,
    options_end,
    options_windows,
    wnd_filter,
}

fn init() -> Result<(), String> {
    Plugin::lock().load();
    Ok(())
}

fn release() {
    Plugin::lock().unload()
}

fn combat(
    event: Option<CombatEvent>,
    src: Option<Agent>,
    dest: Option<Agent>,
    skill_name: Option<&str>,
    id: u64,
    revision: u64,
) {
    Plugin::area_event(event, src, dest, skill_name, id, revision)
}

fn imgui(ui: &Ui, not_loading_or_character_selection: bool) {
    Plugin::render_windows(ui, not_loading_or_character_selection)
}

fn options_windows(ui: &Ui, window_name: Option<&str>) -> bool {
    Plugin::render_window_options(ui, window_name)
}

fn options_end(ui: &Ui) {
    Plugin::lock().render_settings(ui)
}

fn wnd_filter(key: usize, key_down: bool, prev_key_down: bool) -> bool {
    Plugin::key_event(key, key_down, prev_key_down)
}
