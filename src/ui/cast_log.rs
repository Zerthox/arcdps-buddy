use crate::{
    combat::{cast::CastState, CombatData},
    data::{SkillData, SkillHits},
    history::History,
    ui::{format_time, scroll::AutoScroll},
};
use arc_util::{
    colors::{CYAN, GREEN, GREY, RED, YELLOW},
    settings::HasSettings,
    ui::{render, Component, Windowable},
};
use arcdps::{
    exports::{self, Color, Colors, CoreColor},
    imgui::Ui,
};
use serde::{Deserialize, Serialize};
use strum::VariantNames;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CastLog {
    display_time: bool,
    display_duration: bool,
    display_hits: HitDisplay,

    #[serde(skip)]
    scroll: AutoScroll,
}

impl CastLog {
    pub fn new() -> Self {
        Self {
            display_time: true,
            display_duration: true,
            display_hits: HitDisplay::default(),
            scroll: AutoScroll::new(),
        }
    }

    pub fn render_display(&mut self, ui: &Ui) {
        let input_width = render::ch_width(ui, 16);

        ui.checkbox("Display time", &mut self.display_time);
        ui.checkbox("Display duration", &mut self.display_duration);

        let mut index = self.display_hits as usize;
        ui.set_next_item_width(input_width);
        if ui.combo_simple_string("Hits", &mut index, HitDisplay::VARIANTS) {
            self.display_hits = index.into();
        }
    }

    fn format_hits(colors: &Colors, hits: usize, info: &SkillHits) -> (Color, String) {
        let red = colors.core(CoreColor::LightRed).unwrap_or(RED);
        let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
        let blue = colors.core(CoreColor::LightTeal).unwrap_or(CYAN);
        let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

        let max = info.max;
        let has_hits = max > 0;
        let color = if has_hits {
            if hits > max {
                blue
            } else if hits == max {
                green
            } else if hits >= info.expected {
                yellow
            } else {
                red
            }
        } else if max > 0 {
            yellow
        } else {
            red
        };

        let text = if has_hits {
            format!("{hits}/{max}")
        } else {
            format!("{hits}/X")
        };

        (color, text)
    }
}

#[derive(Debug)]
pub struct CastLogProps<'a> {
    pub data: &'a SkillData,
    pub history: &'a mut History<CombatData>,
}

impl Component<CastLogProps<'_>> for CastLog {
    fn render(&mut self, ui: &Ui, props: CastLogProps) {
        let CastLogProps { data, history } = props;

        match history.viewed_fight() {
            Some(fight) if !fight.data.casts.is_empty() => {
                let colors = exports::colors();
                let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
                let red = colors.core(CoreColor::LightRed).unwrap_or(RED);
                let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
                let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

                for cast in &fight.data.casts {
                    if let Some(info) = data.get(cast.skill.id) {
                        if self.display_time {
                            ui.text_colored(grey, format_time(cast.time));
                            ui.same_line();
                        }

                        ui.text(&cast.skill.name);

                        if let Some(hit_info) = info.hits.as_ref() {
                            if let HitDisplay::Target | HitDisplay::Both = self.display_hits {
                                let target_hits = if let Some(species) = fight.target {
                                    cast.hits.iter().filter(|hit| hit.target == species).count()
                                } else {
                                    0
                                };
                                let (color, text) =
                                    Self::format_hits(&colors, target_hits, hit_info);
                                ui.same_line();
                                ui.text_colored(color, text);
                            }

                            let cleave_hits = cast.hits.len();
                            let (color, text) = Self::format_hits(&colors, cleave_hits, hit_info);
                            match self.display_hits {
                                HitDisplay::Cleave => {
                                    ui.same_line();
                                    ui.text_colored(color, text)
                                }
                                HitDisplay::Both => {
                                    ui.same_line();
                                    ui.text_colored(color, format!("({text})"));
                                }
                                _ => {}
                            }
                        }

                        if self.display_duration {
                            let text = format!("{}ms", cast.duration);
                            ui.same_line();
                            match cast.state {
                                CastState::Cancel => ui.text_colored(yellow, text),
                                CastState::Fire => ui.text_colored(green, text),
                                CastState::Interrupt => ui.text_colored(red, text),
                                CastState::Unknown | CastState::Casting | CastState::Pre => {
                                    ui.text("?ms")
                                }
                            }
                        }
                    }
                }
            }
            _ => ui.text("No casts"),
        }

        self.scroll.update(ui);
    }
}

impl Default for CastLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Windowable<CastLogProps<'_>> for CastLog {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, props: &mut CastLogProps) {
        ui.menu("History", || props.history.render_select(ui));

        ui.spacing();
        ui.spacing();

        ui.menu("Display", || self.render_display(ui));
    }
}

impl HasSettings for CastLog {
    type Settings = Self;

    const SETTINGS_ID: &'static str = "cast_log";

    fn current_settings(&self) -> Self::Settings {
        self.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        *self = loaded;
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    VariantNames,
    Serialize,
    Deserialize,
)]
enum HitDisplay {
    #[default]
    Both,
    Target,
    Cleave,
    None,
}

impl From<usize> for HitDisplay {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Both,
            1 => Self::Target,
            2 => Self::Cleave,
            3 => Self::None,
            _ => Self::default(),
        }
    }
}
