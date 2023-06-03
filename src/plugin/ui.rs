use super::Plugin;
use arc_util::ui::{render, Component, Hideable};
use arcdps::{exports, imgui::Ui};

impl Plugin {
    /// Callback for standalone UI creation.
    pub fn render_windows(ui: &Ui, not_loading: bool) {
        let ui_settings = exports::ui_settings();
        if !ui_settings.hidden && (not_loading || ui_settings.draw_always) {
            let plugin = &mut *Self::lock();
            plugin.cast_log.render(ui, (&plugin.data, &plugin.casts));
        }
    }

    /// Callback for settings UI creation.
    pub fn render_settings(&mut self, ui: &Ui) {
        let _style = render::small_padding(ui);

        render::input_key(
            ui,
            "##castlog-key",
            "Casts Hotkey",
            &mut self.cast_log.hotkey,
        );
    }

    /// Callback for ArcDPS option checkboxes.
    pub fn render_window_options(ui: &Ui, option_name: Option<&str>) -> bool {
        if option_name.is_none() {
            let mut plugin = Self::lock();
            ui.checkbox("Buddy Casts", plugin.cast_log.visible_mut());
        }
        false
    }

    /// Handles a key event.
    pub fn key_event(key: usize, down: bool, prev_down: bool) -> bool {
        if down && !prev_down {
            // check for hotkeys
            let mut plugin = Self::lock();
            if matches!(plugin.cast_log.hotkey, Some(hotkey) if hotkey as usize == key) {
                plugin.cast_log.toggle_visibility();
                return false;
            }
        }
        true
    }
}
