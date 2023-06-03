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
    exports::{self, CoreColor},
    imgui::Ui,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastLog {
    pub hotkey: Option<u32>,

    #[serde(skip)]
    last_scroll_max: f32,
}

impl CastLog {
    pub const fn new() -> Self {
        Self {
            hotkey: None,
            last_scroll_max: 0.0,
        }
    }
}

pub type CastLogProps<'a> = (&'a SkillData, &'a [Cast]);

impl Component<CastLogProps<'_>> for CastLog {
    fn render(&mut self, ui: &Ui, (data, casts): CastLogProps) {
        if casts.is_empty() {
            ui.text("No casts");
        } else {
            let colors = exports::colors();
            let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
            let red = colors.core(CoreColor::LightRed).unwrap_or(RED);
            let green = colors.core(CoreColor::LightGreen).unwrap_or(GREEN);
            let blue = colors.core(CoreColor::LightTeal).unwrap_or(CYAN);
            let yellow = colors.core(CoreColor::LightYellow).unwrap_or(YELLOW);

            for cast in casts {
                if let Some(def) = data.get(cast.skill.id) {
                    ui.text_colored(
                        grey,
                        format!("{:>3}.{:03}", cast.time / 1000, cast.time % 1000),
                    );

                    ui.same_line();
                    ui.text(&cast.skill.name);

                    if let Some(hits) = def.hits {
                        let has_hits = hits > 0;
                        let color = if has_hits && cast.hits > hits {
                            blue
                        } else if has_hits && cast.hits == hits {
                            green
                        } else if cast.hits >= def.expected.unwrap_or((hits + 1) / 2) {
                            yellow
                        } else {
                            red
                        };
                        ui.same_line();
                        ui.text_colored(
                            color,
                            if has_hits {
                                format!("{}/{}", cast.hits, hits)
                            } else {
                                format!("{}/X", cast.hits)
                            },
                        );
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

impl Windowable<CastLogProps<'_>> for CastLog {
    const CONTEXT_MENU: bool = true;
}

impl HasSettings for CastLog {
    type Settings = CastLog;

    const SETTINGS_ID: &'static str = "cast_log";

    fn current_settings(&self) -> Self::Settings {
        self.clone()
    }

    fn load_settings(&mut self, loaded: Self::Settings) {
        self.hotkey = loaded.hotkey;
    }
}
