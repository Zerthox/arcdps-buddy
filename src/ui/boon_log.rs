use crate::{
    combat::CombatData,
    history::History,
    ui::{format_time, scroll::AutoScroll},
};
use arc_util::{
    colors::{GREEN, GREY, YELLOW},
    settings::HasSettings,
    ui::{Component, Windowable},
};
use arcdps::{
    exports::{self, CoreColor},
    imgui::Ui,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BoonLog {
    display_time: bool,
    display_duration: bool,

    #[serde(skip)]
    scroll: AutoScroll,
}

impl BoonLog {
    pub const fn new() -> Self {
        Self {
            display_time: true,
            display_duration: true,
            scroll: AutoScroll::new(),
        }
    }

    pub fn render_display(&mut self, ui: &Ui) {
        ui.checkbox("Display time", &mut self.display_time);
        ui.checkbox("Display duration", &mut self.display_duration);
    }
}

#[derive(Debug)]
pub struct BoonLogProps<'a> {
    pub history: &'a mut History<CombatData>,
}

impl Component<BoonLogProps<'_>> for BoonLog {
    fn render(&mut self, ui: &Ui, props: BoonLogProps) {
        let BoonLogProps { history } = props;

        match history.viewed_fight() {
            Some(fight) if !fight.data.boons.is_empty() => {
                let colors = exports::colors();
                let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
                let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
                let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

                for apply in &fight.data.boons {
                    if self.display_time {
                        ui.text_colored(grey, format_time(apply.time));
                        ui.same_line();
                    }

                    ui.text(apply.boon.as_ref());

                    if self.display_duration {
                        ui.same_line();
                        ui.text(format!("{}ms", apply.duration));
                    }

                    ui.same_line();
                    ui.text_colored(if apply.to_player { green } else { yellow }, &apply.target);
                }
            }
            _ => ui.text("No boons"),
        }

        self.scroll.update(ui);
    }
}

impl Default for BoonLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Windowable<BoonLogProps<'_>> for BoonLog {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, props: &mut BoonLogProps) {
        ui.menu("History", || props.history.render_select(ui));

        ui.spacing();
        ui.spacing();

        ui.menu("Display", || self.render_display(ui));
    }
}

impl HasSettings for BoonLog {
    type Settings = Self;

    const SETTINGS_ID: &'static str = "boon_log";

    fn current_settings(&self) -> Self::Settings {
        self.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        *self = loaded;
    }
}
