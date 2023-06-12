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
                    StateChange::LogStart => {
                        let species = event.src_agent as u32;
                        debug!("log start for {species} {dst:?}");
                        let Plugin {
                            start,
                            casts,
                            cast_log,
                            boon_log,
                            ..
                        } = &mut *Self::lock(); // for borrowing
                        *start = Some(event.time);
                        casts
                            .history
                            .add_fight_with_target(event.time, species, dst);
                        cast_log.view.update(&casts.history);
                        boon_log.view.update(&casts.history);
                    }

                    StateChange::LogNPCUpdate => {
                        let species = event.src_agent as u32;
                        debug!("log target change to {species} {dst:?}");
                        let mut plugin = Self::lock();
                        plugin.casts.history.update_latest_target(species, dst);
                    }

                    StateChange::LogEnd => {
                        let species = event.src_agent as u32;
                        debug!("log end for {species} {dst:?}");
                        let mut plugin = Self::lock();
                        plugin.start = None;
                        plugin.casts.history.end_latest_fight(event.time);
                    }

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
                                                plugin.boons.apply(
                                                    event.time,
                                                    event.skill_id,
                                                    &dst,
                                                );
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
}
