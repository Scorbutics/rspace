use crate::{components::spawner::{SpawnerComponent, SpawnerType}, core::{common::GameServices, ecs::EntityId}, factory, systems::ai::AISystem};

use super::phases::LevelPhase;

pub struct Level1Start {
	spawner: EntityId
}

impl Level1Start {
	pub fn new() -> Self {
		Level1Start {
			spawner: 0
		}
	}
}

impl LevelPhase for Level1Start {
	fn on_enter<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		let width = 16 * 4;
		let height = 16 * 4;
		let spawn_pos = factory::random_outside_spawn_pos(game_services.draw_context.screen_width(), game_services.draw_context.screen_height());
		self.spawner = factory::create_entity("",  spawn_pos.0 as i32, spawn_pos.1 as i32, width, height, game_services);
		let mut spawner_component = SpawnerComponent::new(SpawnerType::POINT, 3000, 50.0, 3, 1.0, std::f32::consts::PI * 2.0, super::phases::TrajectoryType::BasicLinear);
		spawner_component.countdown = 5;
		game_services.get_world_mut().add_component::<SpawnerComponent>(&self.spawner, spawner_component);
	}

	fn update<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>) -> bool {
		game_services.get_world().get_system_base::<AISystem>().unwrap().upgrade().unwrap().read().unwrap().len_entities() > 0 || game_services.get_world().is_alive(&self.spawner)
	}
}
