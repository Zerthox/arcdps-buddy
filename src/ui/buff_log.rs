use crate::{
    combat::CombatData,
    history::History,
    ui::{format_time, scroll::AutoScroll},
};
use arc_util::{
    colors::{CYAN, GREEN, GREY, YELLOW},
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
pub struct BuffLog {
    display_time: bool,
    display_duration: bool,

    #[serde(skip)]
    scroll: AutoScroll,
}

impl BuffLog {
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
pub struct BuffLogProps<'a> {
    pub history: &'a mut History<CombatData>,
}

impl Component<BuffLogProps<'_>> for BuffLog {
    fn render(&mut self, ui: &Ui, props: BuffLogProps) {
        let BuffLogProps { history } = props;

        match history.viewed_fight() {
            Some(fight) if !fight.data.buffs.is_empty() => {
                let colors = exports::colors();
                let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
                let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
                let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);
                let blue = colors.core(CoreColor::LightTeal).unwrap_or(CYAN);

                for apply in &fight.data.buffs {
                    if self.display_time {
                        ui.text_colored(grey, format_time(apply.time));
                        ui.same_line();
                    }

                    ui.text(apply.buff.as_ref());

                    if self.display_duration {
                        ui.same_line();
                        ui.text_colored(
                            yellow,
                            format!("{}.{}s", apply.duration / 1000, apply.duration % 1000),
                        );
                    }

                    ui.same_line();
                    ui.text_colored(
                        if apply.to_player() { blue } else { green },
                        &apply.target.name,
                    );
                }
            }
            _ => ui.text("No buffs"),
        }

        self.scroll.update(ui);
    }
}

impl Default for BuffLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Windowable<BuffLogProps<'_>> for BuffLog {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, props: &mut BuffLogProps) {
        ui.menu("History", || props.history.render_select(ui));

        ui.spacing();
        ui.spacing();

        ui.menu("Display", || self.render_display(ui));
    }
}

impl HasSettings for BuffLog {
    type Settings = Self;

    const SETTINGS_ID: &'static str = "buff_log";

    fn current_settings(&self) -> Self::Settings {
        self.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        *self = loaded;
    }
}
