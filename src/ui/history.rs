use crate::history::{Fight, History};
use arc_util::{colors::GREY, ui::Component};
use arcdps::{
    exports::{self, CoreColor},
    imgui::Ui,
};

#[derive(Debug, Clone)]
pub struct HistoryView {
    viewed: usize,
}

impl HistoryView {
    pub const fn new() -> Self {
        Self { viewed: 0 }
    }

    pub fn fight<'a, T>(&self, history: &'a History<T>) -> Option<&'a Fight<T>> {
        history.fight_at(self.viewed)
    }

    pub fn update<T>(&mut self, history: &History<T>) {
        if self.viewed != 0 {
            self.viewed += 1;
            if self.viewed >= history.len() {
                self.viewed = 0;
            }
        }
    }
}

impl Default for HistoryView {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Component<&History<T>> for HistoryView {
    fn render(&mut self, ui: &Ui, history: &History<T>) {
        let colors = exports::colors();

        if history.is_empty() {
            ui.text("No history");
        } else {
            let grey = colors.core(CoreColor::MediumGrey).unwrap_or(GREY);
            for (i, fight) in history.all_fights().enumerate() {
                // TODO: display log start time
                let text = match fight.duration() {
                    Some(duration) => format!("{} ({}s)", fight.name, duration / 1000),
                    None => format!("{} (?s)", fight.name),
                };
                if i == self.viewed {
                    ui.text(text);
                } else {
                    ui.text_colored(grey, text);
                    if ui.is_item_clicked() {
                        self.viewed = i;
                    }
                }
            }
        }
    }
}
