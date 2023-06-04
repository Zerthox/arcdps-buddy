use crate::{
    cast::{Cast, CastState},
    data::SkillData,
};
use arc_util::{
    colors::{CYAN, GREEN, GREY, RED, YELLOW},
    settings::HasSettings,
    ui::{Component, Windowable},
};
use arcdps::{
    exports::{self, Color, Colors, CoreColor},
    imgui::Ui,
};
use serde::{Deserialize, Serialize};
use strum::{EnumVariantNames, VariantNames};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CastLog {
    pub hotkey: Option<u32>,
    display_time: bool,
    display_hits: HitDisplay,

    #[serde(skip)]
    last_scroll_max: f32,
}

impl CastLog {
    pub fn new() -> Self {
        Self {
            hotkey: None,
            display_time: true,
            display_hits: HitDisplay::default(),
            last_scroll_max: 0.0,
        }
    }

    fn format_hits(
        colors: &Colors,
        hits: usize,
        max: u32,
        expected: Option<u32>,
    ) -> (Color, String) {
        let red = colors.core(CoreColor::LightRed).unwrap_or(RED);
        let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
        let blue = colors.core(CoreColor::LightTeal).unwrap_or(CYAN);
        let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

        let max = max as usize;
        let has_hits = max > 0;
        let color = if has_hits {
            if hits > max {
                blue
            } else if hits == max {
                green
            } else if hits
                >= expected
                    .map(|expected| expected as usize)
                    .unwrap_or((max + 1) / 2)
            {
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

#[derive(Debug, Clone)]
pub struct CastLogProps<'a> {
    pub data: &'a SkillData,
    pub casts: &'a [Cast],
    pub target: Option<u32>,
}

impl Component<CastLogProps<'_>> for CastLog {
    fn render(&mut self, ui: &Ui, props: CastLogProps) {
        let CastLogProps {
            data,
            casts,
            target,
        } = props;

        if casts.is_empty() {
            ui.text("No casts");
        } else {
            let colors = exports::colors();
            let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
            let red = colors.core(CoreColor::LightRed).unwrap_or(RED);
            let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
            let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

            for cast in casts {
                if let Some(def) = data.get(cast.skill.id) {
                    if self.display_time {
                        ui.text_colored(
                            grey,
                            format!("{:>3}.{:03}", cast.time / 1000, cast.time % 1000),
                        );
                    }

                    ui.same_line();
                    ui.text(&cast.skill.name);

                    if let Some(max) = def.hits {
                        if let HitDisplay::Target | HitDisplay::Both = self.display_hits {
                            let target_hits = if let Some(target) = target {
                                cast.hits.iter().filter(|hit| hit.target == target).count()
                            } else {
                                0
                            };
                            let (color, text) =
                                Self::format_hits(&colors, target_hits, max, def.expected);
                            ui.same_line();
                            ui.text_colored(color, text);
                        }

                        let cleave_hits = cast.hits.len();
                        let (color, text) =
                            Self::format_hits(&colors, cleave_hits, max, def.expected);
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

                    let text = format!("{}ms", cast.duration);
                    ui.same_line();
                    match cast.state {
                        CastState::Unknown => ui.text("?ms"),
                        CastState::Cancel => ui.text_colored(yellow, text),
                        CastState::Fire => ui.text_colored(green, text),
                        CastState::Interrupt => ui.text_colored(red, text),
                    }
                }
            }
        }

        // auto scroll
        #[allow(clippy::float_cmp)]
        if ui.scroll_y() == self.last_scroll_max {
            ui.set_scroll_here_y_with_ratio(1.0);
        }
        self.last_scroll_max = ui.scroll_max_y();
    }
}

impl Default for CastLog {
    fn default() -> Self {
        Self::new()
    }
}

impl Windowable<CastLogProps<'_>> for CastLog {
    const CONTEXT_MENU: bool = true;

    fn render_menu(&mut self, ui: &Ui, _props: &CastLogProps) {
        ui.menu("Display", || {
            ui.checkbox("Time", &mut self.display_time);

            let mut index = self.display_hits as usize;
            if ui.combo_simple_string("Hits", &mut index, HitDisplay::VARIANTS) {
                self.display_hits = index.into();
            }
        });
    }
}

impl HasSettings for CastLog {
    type Settings = CastLog;

    const SETTINGS_ID: &'static str = "cast_log";

    fn current_settings(&self) -> Self::Settings {
        self.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        let scroll = self.last_scroll_max;
        *self = loaded;
        self.last_scroll_max = scroll;
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
    EnumVariantNames,
    Serialize,
    Deserialize,
)]
enum HitDisplay {
    #[default]
    Both,
    Target,
    Cleave,
}

impl From<usize> for HitDisplay {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Both,
            1 => Self::Target,
            2 => Self::Cleave,
            _ => Self::default(),
        }
    }
}
