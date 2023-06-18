use super::{
    boon_log::{BoonLog, BoonLogProps},
    cast_log::{CastLog, CastLogProps},
    history::HistoryView,
};
use crate::{combat::FightData, data::SkillData, history::History};
use arc_util::{
    settings::HasSettings,
    ui::{Component, Windowable},
};
use arcdps::imgui::{TabBar, TabItem, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct MultiView {
    pub casts: CastLog,
    pub boons: BoonLog,
}

impl MultiView {
    pub fn new() -> Self {
        Self {
            casts: CastLog::new(),
            boons: BoonLog::new(),
        }
    }
}

#[derive(Debug)]
pub struct MultiViewProps<'a> {
    pub data: &'a SkillData,
    pub history: &'a History<FightData>,
    pub view: &'a mut HistoryView,
}

impl Component<MultiViewProps<'_>> for MultiView {
    fn render(&mut self, ui: &Ui, props: MultiViewProps) {
        let MultiViewProps {
            data,
            history,
            view,
        } = props;

        TabBar::new("##tabs").build(ui, || {
            TabItem::new("Casts").build(ui, || {
                self.casts.render(
                    ui,
                    CastLogProps {
                        data,
                        history,
                        view,
                    },
                )
            });

            TabItem::new("Boons")
                .build(ui, || self.boons.render(ui, BoonLogProps { history, view }));
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
        ui.menu("History", || props.view.render(ui, props.history));

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
}

impl HasSettings for MultiView {
    type Settings = MultiViewSettings;

    const SETTINGS_ID: &'static str = "multi_view";

    fn current_settings(&self) -> Self::Settings {
        MultiViewSettings {
            casts: self.casts.current_settings(),
            boons: self.boons.current_settings(),
        }
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        let Self::Settings { casts, boons } = loaded;
        self.casts.load_settings(casts);
        self.boons.load_settings(boons);
    }
}
