use super::Plugin;
use crate::combat::{
    breakbar::BreakbarHit,
    buff::BuffApply,
    cast::{Cast, CastState},
    skill::Skill,
};
use arcdps::{evtc::EventKind, Activation, Agent, CombatEvent, StateChange, Strike};
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
                let is_self = src.is_self != 0;
                match event.kind() {
                    EventKind::StateChange => match event.is_statechange {
                        StateChange::LogStart => Self::lock().start_fight(event, dst),
                        StateChange::LogNPCUpdate => Self::lock().fight_target(event, dst),
                        StateChange::LogEnd => Self::lock().end_fight(event, dst),
                        _ => {}
                    },

                    EventKind::Activation if is_self => {
                        let mut plugin = Self::lock();
                        if let Some(time) = plugin.combat_time(&event) {
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
                        let mut plugin = Self::lock();
                        if is_self || plugin.is_own_minion(&event) {
                            if let (Some(dst), Some(time)) = (dst, plugin.combat_time(&event)) {
                                // ignore applies to self or same agent
                                if dst.is_self == 0 && dst.id != src.id {
                                    // TODO: "effective" duration excluding overstack?
                                    plugin.apply_buff(event.skill_id, &dst, event.value, time);
                                }
                            }
                        }
                    }

                    EventKind::DirectDamage => {
                        let mut plugin = Self::lock();
                        let is_minion = plugin.is_own_minion(&event);
                        if is_self || is_minion {
                            if let (Some(dst), Some(time)) = (dst, plugin.combat_time(&event)) {
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

    fn combat_time(&self, event: &CombatEvent) -> Option<i32> {
        // TODO: add data to previous fight?
        self.start
            .filter(|start| event.time >= *start)
            .map(|start| (event.time - start) as i32)
    }

    fn start_fight(&mut self, event: CombatEvent, dst: Option<Agent>) {
        let species = event.src_agent as u32;
        debug!("log start for {species}, {dst:?}");
        self.start = Some(event.time);
        self.history
            .add_fight_with_target(event.time, species, dst.as_ref());
    }

    fn fight_target(&mut self, event: CombatEvent, dst: Option<Agent>) {
        let species = event.src_agent as u32;
        debug!("log target change to {species}, {dst:?}");
        self.history.update_latest_target(species, dst.as_ref());
    }

    fn end_fight(&mut self, event: CombatEvent, dst: Option<Agent>) {
        let species = event.src_agent;
        debug!("log end for {species}, {dst:?}");
        self.start = None;
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
            let cast = Cast::from_end(time - duration, skill, CastState::Casting, duration);
            debug!("complete without start {cast:?}");
            self.add_cast(cast);
        }
    }

    fn apply_buff(&mut self, buff: u32, target: &Agent, duration: i32, time: i32) {
        if let (Some(fight), Ok(buff)) = (self.history.latest_fight_mut(), buff.try_into()) {
            fight
                .data
                .buffs
                .push(BuffApply::new(time, buff, duration, target))
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
            let hit = BreakbarHit::new(time, skill, damage, target);
            fight.data.breakbar.push(hit);
        }
    }
}
