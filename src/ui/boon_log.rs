use crate::{
    boons::Boons,
    ui::{format_time, history::HistoryView, scroll::AutoScroll},
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
    #[serde(skip)]
    pub view: HistoryView,

    #[serde(skip)]
    scroll: AutoScroll,
}

impl BoonLog {
    pub fn new() -> Self {
        Self {
            view: HistoryView::new(),
            scroll: AutoScroll::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoonLogProps<'a> {
    pub boons: &'a Boons,
}

impl Component<BoonLogProps<'_>> for BoonLog {
    fn render(&mut self, ui: &Ui, props: BoonLogProps) {
        let BoonLogProps { boons: casts } = props;

        match self.view.fight(&casts.history) {
            Some(fight) if !fight.data.is_empty() => {
                let colors = exports::colors();
                let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
                let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
                let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

                for apply in &fight.data {
                    ui.text_colored(grey, format_time(apply.time));
                    ui.text(apply.boon.as_ref());
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

    fn render_menu(&mut self, ui: &Ui, props: &BoonLogProps) {
        let BoonLogProps { boons, .. } = props;

        ui.menu("History", || self.view.render(ui, &boons.history));
    }
}

impl HasSettings for BoonLog {
    type Settings = ();

    const SETTINGS_ID: &'static str = "boon_log";

    fn current_settings(&self) -> Self::Settings {}

    fn load_settings(&mut self, _loaded: Self::Settings) {}
}
