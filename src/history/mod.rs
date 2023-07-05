#![allow(unused)]

mod fight;
mod settings;
mod ui;

pub use self::fight::*;
pub use self::settings::*;

use arcdps::Agent;
use std::{cell::Cell, collections::VecDeque, sync::atomic::AtomicUsize};

#[derive(Debug)]
pub struct History<T> {
    pub settings: HistorySettings,
    viewed: usize,
    fights: VecDeque<Fight<T>>,
}

impl<T> History<T> {
    pub const fn new(max_fights: usize, min_duration: u64) -> Self {
        Self {
            settings: HistorySettings::new(max_fights, min_duration),
            viewed: 0,
            fights: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.fights.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fights.is_empty()
    }

    pub fn viewed(&self) -> usize {
        self.viewed
    }

    pub fn viewed_fight(&self) -> Option<&Fight<T>> {
        self.fight_at(self.viewed)
    }

    pub fn viewed_fight_mut(&mut self) -> Option<&mut Fight<T>> {
        self.fight_at_mut(self.viewed)
    }

    pub fn latest_fight(&self) -> Option<&Fight<T>> {
        self.fights.front()
    }

    pub fn latest_fight_mut(&mut self) -> Option<&mut Fight<T>> {
        self.fights.front_mut()
    }

    pub fn fight_at(&self, index: usize) -> Option<&Fight<T>> {
        self.fights.get(index)
    }

    pub fn fight_at_mut(&mut self, index: usize) -> Option<&mut Fight<T>> {
        self.fights.get_mut(index)
    }

    pub fn all_fights(&self) -> impl Iterator<Item = &Fight<T>> {
        self.fights.iter()
    }

    pub fn add_fight(&mut self, time: u64, data: T)
    where
        T: Default,
    {
        if let Some(prev) = self.fights.front() {
            if matches!(prev.duration(), Some(duration) if duration < self.settings.min_duration) {
                self.fights.pop_front();
            }
        }
        if self.fights.len() > self.settings.max_fights {
            self.fights.pop_back();
        }
        self.fights.push_front(Fight::new(time, data));
        self.update_viewed();
    }

    fn update_viewed(&mut self) {
        if self.viewed != 0 {
            self.viewed += 1;
            if self.viewed >= self.len() {
                self.viewed = 0;
            }
        }
    }

    pub fn add_fight_default(&mut self, time: u64)
    where
        T: Default,
    {
        self.add_fight(time, T::default())
    }

    pub fn add_fight_with_target(&mut self, time: u64, species: u32, target: Option<&Agent>)
    where
        T: Default,
    {
        self.add_fight_default(time);
        self.update_latest_target(species, target);
    }

    pub fn update_latest_target(&mut self, species: u32, target: Option<&Agent>) {
        if let Some(fight) = self.latest_fight_mut() {
            fight.update_target(species, target);
        }
    }

    pub fn end_latest_fight(&mut self, time: u64) {
        if let Some(fight) = self.latest_fight_mut() {
            let duration = fight.end(time);
            if duration < self.settings.min_duration {
                self.fights.pop_front();
                self.update_viewed();
            }
        }
    }
}
