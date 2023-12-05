use super::agent::Target;
use log::debug;

pub use crate::data::Condition;

/// Transfer tracking.
#[derive(Debug, Clone)]
pub struct TransferTracker {
    /// Detected transfers.
    pub transfers: Vec<Transfer>,

    /// Condition removes.
    remove: Vec<Remove>,

    /// Condition applies as transfer candidates.
    apply: Vec<Transfer>,
}

impl TransferTracker {
    /// Time to retain candidates.
    pub const RETAIN_TIME: i32 = 100;

    pub const fn new() -> Self {
        Self {
            transfers: Vec::new(),
            remove: Vec::new(),
            apply: Vec::new(),
        }
    }

    /// Adds a condition remove.
    pub fn add_remove(&mut self, remove: Remove) {
        self.purge(remove.time);
        if let Some(apply) = Self::find_remove(&mut self.apply, |apply| apply.matches(&remove)) {
            debug!("transfer: {remove:?} matches {apply:?}");
            self.transfers.push(apply)
        } else {
            self.remove.push(remove)
        }
    }

    /// Adds a condition apply as transfer candidate.
    pub fn add_apply(&mut self, apply: Transfer) {
        self.purge(apply.time);
        if let Some(remove) = Self::find_remove(&mut self.remove, |remove| apply.matches(remove)) {
            debug!("transfer: {apply:?} matches {remove:?}");
            self.transfers.push(apply)
        } else {
            self.apply.push(apply)
        }
    }

    fn find_remove<T>(vec: &mut Vec<T>, pred: impl FnMut(&T) -> bool) -> Option<T> {
        if let Some(index) = vec.iter().position(pred) {
            Some(vec.swap_remove(index))
        } else {
            None
        }
    }

    /// Purges old information.
    fn purge(&mut self, now: i32) {
        self.remove.retain(|el| Self::check_time(el.time, now));
        self.apply.retain(|el| Self::check_time(el.time, now));
    }

    /// Checks if the time should be kept.
    fn check_time(time: i32, now: i32) -> bool {
        time + Self::RETAIN_TIME >= now
    }
}

impl Default for TransferTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a condition remove.
#[derive(Debug, Clone)]
pub struct Remove {
    /// Time of the remove.
    pub time: i32,

    /// Condition removed.
    pub condi: Condition,

    /// Amount of stacks removed.
    pub stacks: u32,
}

impl Remove {
    /// Creates a new condition transfer.
    pub fn new(time: i32, condi: Condition, stacks: u32) -> Self {
        Self {
            time,
            condi,
            stacks,
        }
    }
}

/// Information about a condition transfer.
#[derive(Debug, Clone)]
pub struct Transfer {
    /// Time of the transfer.
    pub time: i32,

    /// Condition transferred.
    pub condi: Condition,

    /// Amount of stacks transferred.
    pub stacks: u32,

    /// Target the condition was transferred to.
    pub target: Target,
}

impl Transfer {
    /// Error margin for transfer times.
    pub const TIME_EPSILON: i32 = 10;

    /// Creates a new condition transfer.
    pub fn new(time: i32, condi: Condition, stacks: u32, target: Target) -> Self {
        Self {
            time,
            condi,
            stacks,

            target,
        }
    }

    /// Check whether the transfer candidate matches a remove.
    pub fn matches(&self, remove: &Remove) -> bool {
        self.condi == remove.condi
            && self.stacks == remove.stacks
            && (self.time - remove.time) <= Self::TIME_EPSILON
    }
}
