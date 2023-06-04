use super::Plugin;
use crate::casts::{Cast, Skill};
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
                        debug!("log start for {species}");
                        let mut plugin = &mut *Self::lock(); // for borrowing
                        plugin.start = Some(event.time);
                        plugin.casts.add_fight(species, dst, event.time);
                        plugin.cast_log.update_viewed(plugin.casts.fight_count());
                    }

                    StateChange::LogNPCUpdate => {
                        let species = event.src_agent as u32;
                        debug!("log target change to {species}");
                        let mut plugin = Self::lock();
                        plugin.casts.update_target(species, dst);
                    }

                    StateChange::LogEnd => {
                        let species = event.src_agent as u32;
                        debug!("log end for {species}");
                        let mut plugin = Self::lock();
                        plugin.start = None;
                        plugin.casts.end_fight(event.time);
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
                                        if event.is_buff_remove == BuffRemove::None
                                            && event.buff == 0
                                        {
                                            if let Ok(
                                                Strike::Normal | Strike::Crit | Strike::Glance,
                                            ) = event.result.try_into()
                                            {
                                                // TODO: use local combat events for hits?
                                                if let Some(dst) = dst {
                                                    let skill =
                                                        plugin.data.map_hit_id(event.skill_id);
                                                    if let Some(cast) =
                                                        plugin.casts.latest_cast_mut(skill)
                                                    {
                                                        cast.hit(&dst);
                                                        debug!("hit {cast:?}");
                                                    } else {
                                                        let skill = Skill::new(skill, skill_name);
                                                        let cast =
                                                            Cast::new(skill, event.time - start);
                                                        debug!("hit without start {cast:?}");
                                                        plugin.casts.add_cast(cast);
                                                    }
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
