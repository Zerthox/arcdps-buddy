use super::{
    boon_log::{BoonLog, BoonLogProps},
    breakbar_log::{BreakbarLog, BreakbarLogProps},
    cast_log::{CastLog, CastLogProps},
};
use crate::{combat::CombatData, data::SkillData, history::History};
use arc_util::{
    settings::HasSettings,
    ui::{Component, Windowable},
};
use arcdps::imgui::{ChildWindow, TabBar, TabItem, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct MultiView {
    pub casts: CastLog,
    pub boons: BoonLog,
    pub breakbars: BreakbarLog,
}

impl MultiView {
    pub fn new() -> Self {
        Self {
            casts: CastLog::new(),
            boons: BoonLog::new(),
            breakbars: BreakbarLog::new(),
        }
    }

    fn scroll_tab(ui: &Ui, name: impl AsRef<str>, render: impl FnOnce()) {
        TabItem::new(name).build(ui, || ChildWindow::new("##scroller").build(ui, render));
    }
}

#[derive(Debug)]
pub struct MultiViewProps<'a> {
    pub data: &'a SkillData,
    pub history: &'a mut History<CombatData>,
}

impl Component<MultiViewProps<'_>> for MultiView {
    fn render(&mut self, ui: &Ui, props: MultiViewProps) {
        let MultiViewProps { data, history } = props;

        TabBar::new("##tabs").build(ui, || {
            Self::scroll_tab(ui, "Casts", || {
                self.casts.render(ui, CastLogProps { data, history })
            });
            Self::scroll_tab(ui, "Boons", || {
                self.boons.render(ui, BoonLogProps { history })
            });
            Self::scroll_tab(ui, "Breakbar", || {
                self.breakbars.render(ui, BreakbarLogProps { history })
            });
        });
    }
}

impl Default for MultiView {
    fn default() -> Self {
        Self::new()
    }
}

impl Windowable<MultiViewProps<'_>> for MultiView {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, props: &mut MultiViewProps) {
        ui.menu("History", || props.history.render_select(ui));

        ui.spacing();
        ui.spacing();

        ui.menu("Casts Display", || self.casts.render_display(ui));
        ui.menu("Boons Display", || self.boons.render_display(ui));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiViewSettings {
    pub casts: <CastLog as HasSettings>::Settings,
    pub boons: <BoonLog as HasSettings>::Settings,
    pub breakbars: <BreakbarLog as HasSettings>::Settings,
}

impl HasSettings for MultiView {
    type Settings = MultiViewSettings;

    const SETTINGS_ID: &'static str = "multi_view";

    fn current_settings(&self) -> Self::Settings {
        MultiViewSettings {
            casts: self.casts.current_settings(),
            boons: self.boons.current_settings(),
            breakbars: self.breakbars.current_settings(),
        }
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        let Self::Settings {
            casts,
            boons,
            breakbars,
        } = loaded;
        self.casts.load_settings(casts);
        self.boons.load_settings(boons);
        self.breakbars.load_settings(breakbars);
    }
}
