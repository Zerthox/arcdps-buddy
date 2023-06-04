use super::Plugin;
use crate::{data::LoadError, ui::CastLogProps};
use arc_util::{
    colors::{GREEN, GREY, RED, YELLOW},
    ui::{render, Component, Hideable},
};
use arcdps::{
    exports::{self, CoreColor},
    imgui::Ui,
};

impl Plugin {
    /// Callback for standalone UI creation.
    pub fn render_windows(ui: &Ui, not_loading: bool) {
        let ui_settings = exports::ui_settings();
        if !ui_settings.hidden && (not_loading || ui_settings.draw_always) {
            let plugin = &mut *Self::lock();
            plugin.cast_log.render(
                ui,
                CastLogProps {
                    data: &plugin.data,
                    casts: &plugin.casts,
                    target: plugin.target,
                },
            );
        }
    }

    /// Callback for settings UI creation.
    pub fn render_settings(&mut self, ui: &Ui) {
        let colors = exports::colors();
        let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
        let red = colors.core(CoreColor::LightRed).unwrap_or(RED);
        let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
        let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

        let _style = render::small_padding(ui);

        ui.text_colored(grey, "Custom definitions");
        render::input_key(
            ui,
            "##castlog-key",
            "Casts Hotkey",
            &mut self.cast_log.hotkey,
        );

        ui.spacing();
        ui.spacing();

        ui.text_colored(grey, "Custom data");
        ui.text("Status:");
        ui.same_line();
        match self.data_state {
            Ok(()) => ui.text_colored(green, "Loaded"),
            Err(LoadError::NotFound) => ui.text_colored(yellow, "Not found"),
            Err(LoadError::FailedToRead) => ui.text_colored(red, "Failed to read file"),
            Err(LoadError::Invalid) => ui.text_colored(red, "Failed to parse"),
        }
        if ui.button("Reload") {
            self.load_data();
        }
        ui.same_line_with_spacing(0.0, 5.0);
        if ui.button("Reset") {
            self.reset_data();
        }
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
