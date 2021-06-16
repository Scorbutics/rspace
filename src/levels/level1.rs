use crate::{core::{common::GameServices}};

use super::{phase_basic_spawn::LevelPhaseBasicSpawn, phases::{LevelPhase, TrajectoryType}};

pub struct Level1Start {
	base: LevelPhaseBasicSpawn
}

//impl LevelPhaseI

impl Level1Start {
	pub fn new() -> Self {
		Level1Start {
			base: LevelPhaseBasicSpawn::new(TrajectoryType::BasicDiagonalLeft, 5)
		}
	}
}

impl LevelPhase for Level1Start {
	fn on_enter<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		self.base.on_enter(game_services)
	}

	fn update<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>) -> bool {
		self.base.update(game_services)
	}
}
