use super::Plugin;
use crate::{cast::Cast, skill::Skill};
use arcdps::{Activation, Agent, BuffRemove, CombatEvent, StateChange};
use log::debug;

impl Plugin {
    fn latest_cast_mut(&mut self, skill: u32) -> Option<&mut Cast> {
        self.casts
            .iter_mut()
            .rev()
            .find(|cast| cast.skill.id == skill)
    }

    fn add_cast(&mut self, cast: Cast) {
        let index = self
            .casts
            .iter()
            .rev()
            .position(|other| other.time <= cast.time)
            .unwrap_or(0);
        self.casts.insert(self.casts.len() - index, cast);
    }

    /// Handles a combat event from area stats.
    pub fn area_event(
        &mut self,
        event: Option<CombatEvent>,
        src: Option<Agent>,
        _dst: Option<Agent>,
        skill_name: Option<&str>,
        _event_id: u64,
        _revision: u64,
    ) {
        if matches!(src, Some(Agent { is_self,..}) if is_self != 0) {
            if let Some(event) = event {
                match event.is_statechange {
                    StateChange::EnterCombat => {
                        debug!("combat enter");
                        self.casts.clear();
                        self.start = Some(event.time);
                    }

                    StateChange::ExitCombat => {
                        debug!("combat exit");
                        self.start = None;
                    }

                    StateChange::None => {
                        if let Some(start) = self.start {
                            if self.data.has_skill(event.skill_id) {
                                match event.is_activation {
                                    Activation::Start => {
                                        let skill = Skill::new(event.skill_id, skill_name);
                                        let cast = Cast::new(skill, event.time - start);
                                        debug!("start {cast:?}");
                                        self.add_cast(cast);
                                    }

                                    activation @ (Activation::CancelFire
                                    | Activation::CancelCancel
                                    | Activation::Reset) => {
                                        let state = activation.into();
                                        let duration = event.value;

                                        if let Some(cast) = self.latest_cast_mut(event.skill_id) {
                                            debug!("complete {cast:?}");
                                            cast.complete(state, duration);
                                        } else {
                                            debug!(
                                                "complete without start {:?} ({}) with {:?}",
                                                skill_name, event.skill_id, activation
                                            )
                                        }
                                    }

                                    Activation::None => {
                                        if event.is_buff_remove == BuffRemove::None
                                            && event.buff == 0
                                        {
                                            // TODO: use local combat events for hits?
                                            if let Some(cast) = self.latest_cast_mut(event.skill_id)
                                            {
                                                cast.hit();
                                                debug!("hit {cast:?}");
                                            } else {
                                                let skill = Skill::new(event.skill_id, skill_name);
                                                let cast = Cast::new(skill, event.time - start);
                                                debug!("hit without start {cast:?}");
                                                self.add_cast(cast);
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
