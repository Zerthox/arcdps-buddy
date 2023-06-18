use super::Plugin;
use crate::{
    data::LoadError,
    ui::{boon_log::BoonLogProps, cast_log::CastLogProps, multi_view::MultiViewProps},
};
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
            let Plugin {
                data,
                history,
                view,
                cast_log,
                boon_log,
                multi_view,
                ..
            } = &mut *Self::lock(); // for borrowing

            cast_log.render(
                ui,
                CastLogProps {
                    data,
                    history,
                    view,
                },
            );
            boon_log.render(ui, BoonLogProps { history, view });

            multi_view.render(
                ui,
                MultiViewProps {
                    data,
                    history,
                    view,
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

        ui.text_colored(grey, "Hotkeys");
        render::input_key(
            ui,
            "##multi-key",
            "Multi",
            &mut self.multi_view.options.hotkey,
        );
        render::input_key(
            ui,
            "##casts-key",
            "Casts",
            &mut self.cast_log.options.hotkey,
        );
        render::input_key(
            ui,
            "##boons-key",
            "Boons",
            &mut self.boon_log.options.hotkey,
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
            ui.checkbox("Buddy Multi", plugin.multi_view.visible_mut());
            ui.checkbox("Buddy Casts", plugin.cast_log.visible_mut());
            ui.checkbox("Buddy Boons", plugin.boon_log.visible_mut());
        }
        false
    }

    /// Handles a key event.
    pub fn key_event(key: usize, down: bool, prev_down: bool) -> bool {
        if down && !prev_down {
            let Plugin {
                multi_view,
                cast_log,
                boon_log,
                ..
            } = &mut *Self::lock();

            // check for hotkeys
            !multi_view.options.key_press(key)
                && !cast_log.options.key_press(key)
                && !boon_log.options.key_press(key)
        } else {
            true
        }
    }
}
