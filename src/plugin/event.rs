use super::Plugin;
use crate::{casts::Cast, skill::Skill};
use arcdps::{Activation, Agent, BuffRemove, CombatEvent, StateChange, Strike};
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
                match event.is_statechange {
                    StateChange::LogStart => Plugin::lock().start_fight(event, dst),
                    StateChange::LogNPCUpdate => Plugin::lock().fight_target(event, dst),
                    StateChange::LogEnd => Plugin::lock().end_fight(event, dst),

                    StateChange::None if is_self => {
                        let mut plugin = Self::lock();
                        if let Some(start) = plugin.start {
                            // TODO: add data to previous fight?
                            if event.time >= start && plugin.data.contains(event.skill_id) {
                                match event.is_activation {
                                    Activation::Start => {
                                        let skill = Skill::new(event.skill_id, skill_name);
                                        let cast = Cast::new(skill, event.time - start);
                                        debug!("start {cast:?}");
                                        plugin.casts.add_cast(cast);
                                    }

                                    activation @ (Activation::CancelFire
                                    | Activation::CancelCancel
                                    | Activation::Reset) => {
                                        let state = activation.into();
                                        let duration = event.value;

                                        if let Some(cast) =
                                            plugin.casts.latest_cast_mut(event.skill_id)
                                        {
                                            debug!("complete {cast:?}");
                                            cast.complete(state, duration);
                                        } else {
                                            let skill = Skill::new(event.skill_id, skill_name);
                                            let mut cast = Cast::new(
                                                skill,
                                                event.time
                                                    - u64::try_from(duration).unwrap_or_default(),
                                            );
                                            cast.complete(state, duration);
                                            debug!("complete without start {cast:?}");
                                            plugin.casts.add_cast(cast);
                                        }
                                    }

                                    Activation::None => {
                                        if let (BuffRemove::None, Some(dst)) =
                                            (event.is_buff_remove, dst)
                                        {
                                            if event.buff != 0 {
                                                if dst.is_self != 0 {
                                                    // TODO: duration applied
                                                    plugin.boons.apply(
                                                        event.time,
                                                        event.skill_id,
                                                        &dst,
                                                    );
                                                }
                                            } else if let Ok(
                                                Strike::Normal | Strike::Crit | Strike::Glance,
                                            ) = event.result.try_into()
                                            {
                                                // TODO: use local combat events for hits?

                                                let skill = plugin.data.map_hit_id(event.skill_id);
                                                if let Some(cast) =
                                                    plugin.casts.latest_cast_mut(skill)
                                                {
                                                    cast.hit(&dst);
                                                    debug!("hit {:?} {dst:?}", cast.skill);
                                                } else {
                                                    let skill = Skill::new(skill, skill_name);
                                                    let mut cast =
                                                        Cast::new(skill, event.time - start);
                                                    cast.hit(&dst);
                                                    debug!("hit without start {cast:?}");
                                                    plugin.casts.add_cast(cast);
                                                }
                                            }
                                        }
                                    }

                                    _ => {}
                                }
                            }
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    fn start_fight(&mut self, event: CombatEvent, dst: Option<Agent>) {
        let species = event.src_agent as u32;
        debug!("log start for {species} {dst:?}");
        self.start = Some(event.time);
        self.casts
            .history
            .add_fight_with_target(event.time, species, dst.as_ref());
        self.boons
            .history
            .add_fight_with_target(event.time, species, dst.as_ref());

        self.cast_log.view.update(&self.casts.history);
        self.boon_log.view.update(&self.boons.history);
    }

    fn fight_target(&mut self, event: CombatEvent, dst: Option<Agent>) {
        let species = event.src_agent as u32;
        debug!("log target change to {species} {dst:?}");
        self.casts
            .history
            .update_latest_target(species, dst.as_ref());
        self.boons
            .history
            .update_latest_target(species, dst.as_ref());
    }

    fn end_fight(&mut self, event: CombatEvent, dst: Option<Agent>) {
        let species = event.src_agent as u32;
        debug!("log end for {species} {dst:?}");
        self.start = None;
        self.casts.history.end_latest_fight(event.time);
        self.boons.history.end_latest_fight(event.time);
    }
}
