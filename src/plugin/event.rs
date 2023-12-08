use super::Plugin;
use crate::combat::{
    breakbar::BreakbarHit,
    buff::{Buff, BuffApply},
    cast::{Cast, CastState},
    skill::Skill,
    transfer::{Apply, Condition, Remove},
};
use arcdps::{evtc::EventKind, Activation, Agent, BuffRemove, CombatEvent, StateChange, Strike};
use log::debug;

impl Plugin {
    /// Handles a combat event from area stats.
    pub fn area_event(
        event: Option<CombatEvent>,
        src: Option<Agent>,
        dst: Option<Agent>,
        skill_name: Option<&str>,
        _event_id: u64,
        _revision: u64,
    ) {
        if let Some(src) = src {
            if let Some(event) = event {
                let src_self = src.is_self != 0;
                match event.kind() {
                    EventKind::StateChange => match event.is_statechange {
                        StateChange::LogStart => Self::lock().start_fight(event, dst),
                        StateChange::LogNPCUpdate => Self::lock().fight_target(event, dst),
                        StateChange::LogEnd => Self::lock().end_fight(event, dst),
                        _ => {}
                    },

                    EventKind::Activation if src_self => {
                        let mut plugin = Self::lock();
                        if let Some(time) = plugin.history.relative_time(event.time) {
                            if plugin.data.contains(event.skill_id) {
                                match event.is_activation {
                                    Activation::Start => {
                                        plugin.cast_start(&event, skill_name, time)
                                    }

                                    Activation::CancelFire
                                    | Activation::CancelCancel
                                    | Activation::Reset => {
                                        plugin.cast_end(&event, skill_name, time)
                                    }

                                    _ => {}
                                }
                            }
                        }
                    }

                    EventKind::BuffApply => {
                        if let Some(dst) = dst {
                            let buff = event.skill_id;
                            if let Ok(buff) = buff.try_into() {
                                // only care about buff applies to other where source and dest are different
                                if dst.is_self == 0 && dst.id != src.id {
                                    Self::lock().apply_buff(&event, buff, &src, &dst)
                                }
                            } else if let Ok(condi) = buff.try_into() {
                                // only care about condi applies from self to other and ignore extensions
                                if src_self && dst.is_self == 0 && event.is_off_cycle == 0 {
                                    Self::lock().apply_condi(&event, condi, &dst)
                                }
                            }
                        }
                    }

                    EventKind::BuffRemove => {
                        if let Some(dst) = dst {
                            // only care about removes from self to self
                            if event.is_buff_remove == BuffRemove::Manual
                                && src_self
                                && dst.is_self != 0
                            {
                                if let Ok(condi) = event.skill_id.try_into() {
                                    Self::lock().remove_buff(&event, condi)
                                }
                            }
                        }
                    }

                    EventKind::DirectDamage => {
                        let mut plugin = Self::lock();
                        let is_minion = plugin.is_own_minion(&event);
                        if src_self || is_minion {
                            if let (Some(dst), Some(time)) =
                                (dst, plugin.history.relative_time(event.time))
                            {
                                plugin.strike(&event, is_minion, skill_name, &dst, time)
                            }
                        }
                    }

                    _ => {}
                }
            } else if let Some(dst) = dst {
                // check for tracking addition
                if dst.is_self != 0 && src.elite == 0 && src.prof != 0 {
                    let mut plugin = Self::lock();
                    let inst_id = dst.id as u16;
                    plugin.self_instance_id = Some(inst_id);
                    debug!("own instance id changed to {}", inst_id);
                }
            }
        }
    }

    fn is_own_minion(&self, event: &CombatEvent) -> bool {
        match self.self_instance_id {
            Some(id) => event.src_master_instance_id == id,
            None => false,
        }
    }

    fn start_fight(&mut self, event: CombatEvent, target: Option<Agent>) {
        let species = event.src_agent as u32;
        debug!("log start for {species}, {target:?}");

        self.history
            .add_fight_with_target(event.time, species, target.as_ref());
    }

    fn fight_target(&mut self, event: CombatEvent, target: Option<Agent>) {
        let species = event.src_agent as u32;
        debug!("log target change to {species}, {target:?}");
        self.history
            .update_fight_target(event.time, species, target.as_ref());
    }

    fn end_fight(&mut self, event: CombatEvent, target: Option<Agent>) {
        let species = event.src_agent;
        debug!("log end for {species}, {target:?}");
        self.history.end_latest_fight(event.time);
    }

    pub fn latest_cast_mut(&mut self, skill: u32) -> Option<&mut Cast> {
        self.history.latest_fight_mut().and_then(|fight| {
            fight
                .data
                .casts
                .iter_mut()
                .rev()
                .find(|cast| cast.skill.id == skill)
        })
    }

    pub fn add_cast(&mut self, cast: Cast) {
        if let Some(fight) = self.history.latest_fight_mut() {
            let casts = &mut fight.data.casts;
            let index = casts
                .iter()
                .rev()
                .position(|other| other.time <= cast.time)
                .unwrap_or(0);
            casts.insert(casts.len() - index, cast);
        }
    }

    fn cast_start(&mut self, event: &CombatEvent, skill_name: Option<&str>, time: i32) {
        let skill = Skill::new(event.skill_id, skill_name);
        debug!("start {skill:?}");
        let cast = Cast::from_start(time, skill, CastState::Casting);
        self.add_cast(cast);
    }

    fn cast_end(&mut self, event: &CombatEvent, skill_name: Option<&str>, time: i32) {
        let state = event.is_activation.into();
        let duration = event.value;

        let skill = Skill::new(event.skill_id, skill_name);
        if let Some(cast) = self.latest_cast_mut(event.skill_id) {
            cast.complete(skill, state, duration, time);
            debug!("complete {cast:?}");
        } else {
            let cast = Cast::from_end(time - duration, skill, state, duration);
            debug!("complete without start {cast:?}");
            self.add_cast(cast);
        }
    }

    fn apply_buff(&mut self, event: &CombatEvent, buff: Buff, src: &Agent, dst: &Agent) {
        if src.is_self != 0 || self.is_own_minion(event) {
            if let Some((time, fight)) = self.history.fight_and_time(event.time) {
                // TODO: "effective" duration excluding overstack?
                let duration = event.value;
                let apply = BuffApply::new(time, buff, duration, dst.into());
                fight.data.buffs.push(apply)
            }
        }
    }

    fn apply_condi(&mut self, event: &CombatEvent, condi: Condition, target: &Agent) {
        if let Some((time, fight)) = self.history.fight_and_time(event.time) {
            let apply = Apply::new(time, condi, event.value, target.into());
            fight.data.transfers.add_apply(apply);
        }
    }

    fn remove_buff(&mut self, event: &CombatEvent, condi: Condition) {
        if let Some((time, fight)) = self.history.fight_and_time(event.time) {
            let remove = Remove::new(time, condi, event.value);
            fight.data.transfers.add_remove(remove)
        }
    }

    fn strike(
        &mut self,
        event: &CombatEvent,
        is_minion: bool,
        skill_name: Option<&str>,
        target: &Agent,
        time: i32,
    ) {
        let skill = Skill::new(event.skill_id, skill_name);
        match event.result.try_into() {
            Ok(Strike::Normal | Strike::Crit | Strike::Glance) => {
                self.damage_hit(is_minion, skill, target, time)
            }
            Ok(Strike::Breakbar) => self.breakbar_hit(skill, target, event.value, time),
            _ => {}
        }
    }

    fn damage_hit(&mut self, is_minion: bool, mut skill: Skill, target: &Agent, time: i32) {
        // TODO: use local combat events for hits?
        if let Some(info) = self.data.get(skill.id) {
            if info.minion || !is_minion {
                // replace skill id
                skill.id = info.id;

                let max = info.max_duration;
                match self.latest_cast_mut(skill.id) {
                    Some(cast) if time - cast.time <= max => {
                        cast.hit(target);
                        debug!("hit {:?}, {target:?}", cast.skill);
                    }
                    _ => {
                        let cast = Cast::from_hit(time, skill, target);
                        debug!("hit without start {:?}, {target:?}", cast.skill);
                        self.add_cast(cast);
                    }
                }
            }
        }
    }

    fn breakbar_hit(&mut self, skill: Skill, target: &Agent, damage: i32, time: i32) {
        // TODO: display minion indicator?
        if let Some(fight) = self.history.latest_fight_mut() {
            debug!("breakbar {damage} {skill:?} {target:?}");
            let hit = BreakbarHit::new(time, skill, damage, target.into());
            fight.data.breakbar.push(hit);
        }
    }
}
