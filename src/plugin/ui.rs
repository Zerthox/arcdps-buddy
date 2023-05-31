use super::Plugin;
use arc_util::ui::{render, Component, Hideable};
use arcdps::{exports, imgui::Ui};

impl Plugin {
    /// Callback for standalone UI creation.
    pub fn render_windows(&mut self, ui: &Ui, not_loading: bool) {
        let ui_settings = exports::ui_settings();
        if !ui_settings.hidden && (not_loading || ui_settings.draw_always) {
            self.cast_log.render(ui, (&self.data, &self.casts));
        }
    }

    /// Callback for settings UI creation.
    pub fn render_settings(&mut self, ui: &Ui) {
        render::input_key(
            ui,
            "##castlog-key",
            "Casts Hotkey",
            &mut self.cast_log.hotkey,
        );
    }

    /// Callback for ArcDPS option checkboxes.
    pub fn render_window_options(&mut self, ui: &Ui, option_name: Option<&str>) -> bool {
        if option_name.is_none() {
            ui.checkbox("Buddy Casts", self.cast_log.visible_mut());
        }
        false
    }

    /// Handles a key event.
    pub fn key_event(&mut self, key: usize, down: bool, prev_down: bool) -> bool {
        if down && !prev_down {
            // check for hotkeys
            if matches!(self.cast_log.hotkey, Some(hotkey) if hotkey as usize == key) {
                self.cast_log.toggle_visibility();
                return false;
            }
        }
        true
    }
}
