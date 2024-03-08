use super::Plugin;
use crate::combat::{
    breakbar::BreakbarHit,
    buff::{Buff, BuffApply},
    cast::{Cast, CastState},
    player::Player,
    skill::Skill,
    transfer::{Apply, Condition, Remove},
};
use arcdps::{evtc::EventCategory, Activation, Agent, BuffRemove, Event, StateChange, Strike};
use log::debug;

impl Plugin {
    /// Handles a combat event from area stats.
    pub fn area_event(
        event: Option<&Event>,
        src: Option<&Agent>,
        dst: Option<&Agent>,
        skill_name: Option<&str>,
        _event_id: u64,
        _revision: u64,
    ) {
        if let Some(src) = src {
            if let Some(event) = event {
                let src_self = src.is_self != 0;
                match event.categorize() {
                    EventCategory::StateChange => match event.get_statechange() {
                        StateChange::LogStart => Self::lock().start_fight(event, dst),
                        StateChange::LogNPCUpdate => Self::lock().fight_target(event, dst),
                        StateChange::LogEnd => Self::lock().end_fight(event, dst),
                        _ => {}
                    },

                    EventCategory::Activation if src_self => {
                        let mut plugin = Self::lock();
                        if let Some(time) = plugin.history.relative_time(event.time) {
                            if plugin.data.contains(event.skill_id) {
                                match event.get_activation() {
                                    Activation::Start => plugin.cast_start(event, skill_name, time),
                                    Activation::CancelFire
                                    | Activation::CancelCancel
                                    | Activation::Reset => plugin.cast_end(event, skill_name, time),
                                    _ => {}
                                }
                            }
                        }
                    }

                    EventCategory::BuffApply => {
                        if let Some(dst) = dst {
                            let buff = event.skill_id;
                            if let Ok(buff) = buff.try_into() {
                                // only care about buff applies to other where source and dest are different
                                if dst.is_self == 0 && dst.id != src.id {
                                    Self::lock().apply_buff(event, buff, src, dst)
                                }
                            } else if let Ok(condi) = buff.try_into() {
                                // only care about condi applies from self to other and ignore extensions
                                if src_self && dst.is_self == 0 && event.is_offcycle == 0 {
                                    Self::lock().apply_condi(event, condi, dst)
                                }
                            }
                        }
                    }

                    EventCategory::BuffRemove => {
                        if let Some(dst) = dst {
                            // only care about removes from self to self
                            if event.get_buffremove() == BuffRemove::Manual
                                && src_self
                                && dst.is_self != 0
                            {
                                if let Ok(condi) = event.skill_id.try_into() {
                                    Self::lock().remove_buff(event, condi)
                                }
                            }
                        }
                    }

                    EventCategory::Strike => {
                        let mut plugin = Self::lock();
                        if let (Some(dst), Some(time)) =
                            (dst, plugin.history.relative_time(event.time))
                        {
                            plugin.strike(event, skill_name, src, dst, time)
                        }
                    }

                    _ => {}
                }
            } else if let Some(dst) = dst {
                // check for tracking addition
                if src.elite == 0 && src.prof != 0 {
                    let mut plugin = Self::lock();
                    if src.prof != 0 {
                        // player added
                        let player = Player::from_tracking_change(src, dst);
                        if dst.is_self != 0 {
                            plugin.self_instance_id = Some(player.instance_id);
                            debug!("own instance id changed to {}", player.instance_id);
                        }
                        plugin.players.push(player);
                    } else if let Some(pos) =
                        plugin.players.iter().position(|player| player.id == src.id)
                    {
                        // player tracked & removed
                        plugin.players.swap_remove(pos);
                    }
                }
            }
        }
    }

    fn get_master(&self, event: &Event) -> Option<&crate::combat::player::Player> {
        self.players
            .iter()
            .find(|player| player.instance_id == event.src_master_instance_id)
    }

    fn is_own_minion(&self, event: &Event) -> bool {
        match self.self_instance_id {
            Some(id) => event.src_master_instance_id == id,
            None => false,
        }
    }

    fn start_fight(&mut self, event: &Event, target: Option<&Agent>) {
        let species = event.src_agent as u32;
        debug!("log start for {species}, {target:?}");
        self.history
            .add_fight_with_target(event.time, species, target);
    }

    fn fight_target(&mut self, event: &Event, target: Option<&Agent>) {
        let species = event.src_agent as u32;
        debug!("log target change to {species}, {target:?}");
        self.history
            .update_fight_target(event.time, species, target);
    }

    fn end_fight(&mut self, event: &Event, target: Option<&Agent>) {
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

    fn cast_start(&mut self, event: &Event, skill_name: Option<&str>, time: i32) {
        let skill = Skill::new(event.skill_id, skill_name);
        debug!("start {skill:?}");
        let cast = Cast::from_start(time, skill, CastState::Casting);
        self.add_cast(cast);
    }

    fn cast_end(&mut self, event: &Event, skill_name: Option<&str>, time: i32) {
        let state = event.get_activation().into();
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

    fn apply_buff(&mut self, event: &Event, buff: Buff, src: &Agent, dst: &Agent) {
        if src.is_self != 0 || self.is_own_minion(event) {
            if let Some((time, fight)) = self.history.fight_and_time(event.time) {
                // TODO: "effective" duration excluding overstack?
                let duration = event.value;
                let apply = BuffApply::new(time, buff, duration, dst.into());
                fight.data.buffs.push(apply)
            }
        }
    }

    fn apply_condi(&mut self, event: &Event, condi: Condition, target: &Agent) {
        if let Some((time, fight)) = self.history.fight_and_time(event.time) {
            let apply = Apply::new(time, condi, event.value, target.into());
            fight.data.transfers.add_apply(apply);
        }
    }

    fn remove_buff(&mut self, event: &Event, condi: Condition) {
        if let Some((time, fight)) = self.history.fight_and_time(event.time) {
            let remove = Remove::new(time, condi, event.value);
            fight.data.transfers.add_remove(remove)
        }
    }

    fn strike(
        &mut self,
        event: &Event,
        skill_name: Option<&str>,
        attacker: &Agent,
        target: &Agent,
        time: i32,
    ) {
        let skill = Skill::new(event.skill_id, skill_name);
        let is_minion = self.is_own_minion(event);
        let is_own = attacker.is_self != 0 || is_minion;
        match event.get_strike() {
            Strike::Normal | Strike::Crit | Strike::Glance => {
                if is_own {
                    self.damage_hit(is_minion, skill, target, time)
                }
            }
            Strike::Breakbar => {
                let attacker = self
                    .get_master(event)
                    .map(|player| player.into())
                    .unwrap_or(attacker.into());
                self.breakbar_hit(skill, attacker, is_own, target, event.value, time)
            }
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

    fn breakbar_hit(
        &mut self,
        skill: Skill,
        attacker: crate::combat::Agent,
        is_own: bool,
        target: &Agent,
        damage: i32,
        time: i32,
    ) {
        // TODO: minion indicator?
        if let Some(fight) = self.history.latest_fight_mut() {
            debug!("breakbar {damage} {skill:?} from {attacker:?} to {target:?}");
            let hit = BreakbarHit::new(time, skill, damage, attacker, is_own, target.into());
            fight.data.breakbar.push(hit);
        }
    }
}
