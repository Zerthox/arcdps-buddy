mod combat;
mod data;
mod history;
mod plugin;
mod ui;

use plugin::Plugin;

// create exports for arcdps
arcdps::export! {
    name: "Buddy",
    sig: 0x84c13713,
    init: || {
        Plugin::lock().load();
        Ok(())
    },
    release: || Plugin::lock().unload(),
    combat: Plugin::area_event,
    imgui: Plugin::render,
    options_end: |ui| Plugin::lock().render_settings(ui),
    options_windows: Plugin::render_window_options,
    wnd_filter: Plugin::key_event,
}
