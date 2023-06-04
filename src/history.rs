#![allow(unused)]

use arcdps::Agent;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct History<T> {
    fights: VecDeque<Fight<T>>,
}

impl<T> History<T> {
    pub const fn new() -> Self {
        Self {
            fights: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.fights.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fights.is_empty()
    }

    pub fn all_fights(&self) -> impl Iterator<Item = &Fight<T>> {
        self.fights.iter()
    }

    pub fn fight_at(&self, index: usize) -> Option<&Fight<T>> {
        self.fights.get(index)
    }

    pub fn fight_at_mut(&mut self, index: usize) -> Option<&mut Fight<T>> {
        self.fights.get_mut(index)
    }

    pub fn latest_fight(&self) -> Option<&Fight<T>> {
        self.fights.front()
    }

    pub fn latest_fight_mut(&mut self) -> Option<&mut Fight<T>> {
        self.fights.front_mut()
    }

    pub fn add_fight(&mut self, time: u64, max: usize)
    where
        T: Default,
    {
        if self.fights.len() > max {
            self.fights.pop_back();
        }
        self.fights.push_front(Fight::new(time));
    }

    pub fn update_latest_target(&mut self, species: u32, target: Option<Agent>) {
        if let Some(fight) = self.latest_fight_mut() {
            fight.update_target(species, target);
        }
    }

    pub fn end_latest_fight(&mut self, time: u64) {
        if let Some(fight) = self.latest_fight_mut() {
            fight.end(time);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Fight<T> {
    pub id: Option<u32>,
    pub name: String,
    pub start: u64,
    pub end: u64,
    pub data: T,
}

impl<T> Fight<T> {
    pub fn new(start: u64) -> Self
    where
        T: Default,
    {
        Self {
            id: None,
            name: Self::DEFAULT_NAME.into(),
            start,
            end: 0,
            data: T::default(),
        }
    }

    const DEFAULT_NAME: &str = "Unknown";

    pub fn update_target(&mut self, species: u32, target: Option<Agent>) {
        if species > 2 {
            self.id = Some(species);
            self.name = match target.and_then(|agent| agent.name) {
                Some(name) if !name.is_empty() => name.into(),
                _ => Self::DEFAULT_NAME.into(),
            };
        } else {
            self.id = None;
            self.name = Self::DEFAULT_NAME.into();
        }
    }

    pub fn duration(&self) -> u64 {
        if self.end != 0 {
            self.end - self.start
        } else {
            0
        }
    }

    pub fn end(&mut self, time: u64) {
        self.end = time;
    }
}
