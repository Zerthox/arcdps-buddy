use super::{
    boon_log::{BoonLog, BoonLogProps},
    cast_log::{CastLog, CastLogProps},
};
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
    selected: Tab,
}

impl MultiView {
    pub fn new() -> Self {
        Self {
            casts: CastLog::new(),
            boons: BoonLog::new(),
            selected: Tab::Casts,
        }
    }
}

pub type MultiViewProps<'a> = (CastLogProps<'a>, BoonLogProps<'a>);

impl Component<MultiViewProps<'_>> for MultiView {
    fn render(&mut self, ui: &Ui, props: MultiViewProps) {
        let (cast_props, boon_props) = props;
        TabBar::new("##tabs").build(ui, || {
            // TODO: propagate history? display settings?
            TabItem::new("Casts").build(ui, || {
                self.selected = Tab::Casts;
                self.casts.render(ui, cast_props);
            });
            TabItem::new("Boons").build(ui, || {
                self.selected = Tab::Boons;
                self.boons.render(ui, boon_props);
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

    fn render_menu(&mut self, ui: &Ui, props: &MultiViewProps) {
        let (cast_props, boon_props) = props;
        match self.selected {
            Tab::Casts => self.casts.render_menu(ui, cast_props),
            Tab::Boons => self.boons.render_menu(ui, boon_props),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tab {
    Casts,
    Boons,
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
