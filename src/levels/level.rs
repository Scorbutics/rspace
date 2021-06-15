use crate::core::common::GameServices;

use super::phases::LevelPhase;

pub struct Level {
	phases: Vec<Box<dyn LevelPhase>>,
	current_phase_index: usize,
	init: bool
}

impl Level {
	pub fn new(phases: Vec<Box<dyn LevelPhase>>) -> Self {
		Level {
			phases: phases,
			current_phase_index: 0,
			init: false
		}
	}

	pub fn update<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) -> bool {
		let not_finished = self.current_phase_index < self.phases.len();
		if not_finished {
			if ! self.init {
				self.phases[self.current_phase_index].on_enter(game_services);
				self.init = true;
			}
			if ! self.phases[self.current_phase_index].update(game_services) {
				self.current_phase_index += 1;
				if self.current_phase_index < self.phases.len() {
					self.phases[self.current_phase_index].on_enter(game_services);
				}
			}
		}
		not_finished
	}
}