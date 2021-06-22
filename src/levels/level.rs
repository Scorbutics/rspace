use std::sync::{Arc};

use crate::{core::{common::GameServices, events::{EventBus, EventBusBase, Observer}}};

use super::phases::LevelPhase;

pub struct Level<L : LevelPhase> {
	phases: Vec<Box<L>>,
	current_phase_index: usize,
	init: bool,
	event_bus: EventBusBase<L>
}

impl<'playing_state, L: LevelPhase> Level<L> {
	pub fn new(phases: Vec<Box<L>>, observer: Arc<Observer<L>>) -> Self {
		let mut event_bus = EventBusBase::new();
		event_bus.register(observer);
		Level {
			phases: phases,
			current_phase_index: 0,
			init: false,
			event_bus: event_bus
		}
	}

	fn notify_current_phase_change(&self) {
		self.event_bus.notify(self.phases[self.current_phase_index].as_ref());
	}

	pub fn update<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) -> bool {
		let not_finished = self.current_phase_index < self.phases.len();
		if not_finished {
			if ! self.init {
				self.phases[self.current_phase_index].on_enter(game_services);
				self.notify_current_phase_change();

				self.init = true;
			} else if ! self.phases[self.current_phase_index].update(game_services) {
				self.current_phase_index += 1;
				if self.current_phase_index < self.phases.len() {
					//println!("NEXT PHASE {}", self.current_phase_index);
					self.phases[self.current_phase_index].on_enter(game_services);
					self.notify_current_phase_change();
				}
			}
		}
		not_finished
	}
}
