use std::sync::{RwLock, Weak};

use crate::{components::spawner::{SpawnerComponent, SpawnerType}, core::{common::GameServices, ecs}, factory, systems::{ai::AISystem, spawner::SpawnMobSystem}};

use super::{phases::{LevelPhase, TrajectoryType}};

#[derive(PartialEq, Eq)]
pub enum SpawnerPositionProperty {
	AlternateSymetric,
	Fixed,
	Random
}

pub struct SpawnerProperties {
	pattern: TrajectoryType,
	density: usize,
	frequency_ms: u32,
	enemy_num: u16,
	shot_frequency_factor: f32,
	pub luck: f32,
	pub position_prop: SpawnerPositionProperty,
}

impl SpawnerProperties {
	pub fn new(pattern: TrajectoryType, density: usize, frequency_ms: u32, enemy_num: u16, shot_frequency_factor: f32) -> Self {
		SpawnerProperties {
			pattern: pattern,
			density: density,
			luck: 100.0,
			frequency_ms: frequency_ms,
			enemy_num: enemy_num,
			shot_frequency_factor: shot_frequency_factor,
			position_prop: SpawnerPositionProperty::Random
		}
	}
}

pub struct LevelPhaseBasicSpawn {
	properties: Vec<SpawnerProperties>,
	pub hyperspace_speed: f64,
	ai_system: Option<Weak<RwLock<ecs::System>>>,
	spawner_system: Option<Weak<RwLock<ecs::System>>>,
}

impl LevelPhaseBasicSpawn {
	pub fn new(hyperspace_speed: f64) -> Self {
		LevelPhaseBasicSpawn {
			properties: Vec::new(),
			hyperspace_speed: hyperspace_speed,
			ai_system: None,
			spawner_system: None
		}
	}

	pub fn add(&mut self, pattern: TrajectoryType, density: usize, frequency_ms: u32, enemy_num: u16, shot_frequency_factor: f32) -> &mut SpawnerProperties {
		self.properties.push(SpawnerProperties::new(pattern, density, frequency_ms, enemy_num, shot_frequency_factor));
		self.properties.last_mut().unwrap()
	}
}

impl LevelPhase for LevelPhaseBasicSpawn {
	fn on_enter<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for properties in &self.properties {
			let area_width = 16 * 4;
			let area_height = 16 * 4;
			let spawn_pos = if properties.position_prop != SpawnerPositionProperty::Fixed {
				factory::random_outside_spawn_pos(game_services.draw_context.screen_width(), game_services.draw_context.screen_height())
			} else {
				((game_services.draw_context.screen_width() / 2) as f32, 180 as f32)
			};
			let spawner = factory::create_entity("",  spawn_pos.0 as i32, spawn_pos.1 as i32, 0, area_width, area_height, game_services);
			let mut spawner_component = SpawnerComponent::new(SpawnerType::POINT, properties.frequency_ms, properties.luck, properties.enemy_num,
				1.0, std::f32::consts::PI * 2.0, properties.pattern);
			spawner_component.countdown = properties.density;
			spawner_component.shot_frequency_factor = properties.shot_frequency_factor;
			spawner_component.randomize_pos = properties.position_prop == SpawnerPositionProperty::Random;
			spawner_component.symetric_alternate_pos = properties.position_prop == SpawnerPositionProperty::AlternateSymetric;
			game_services.get_world_mut().add_component::<SpawnerComponent>(&spawner, spawner_component);
		}
		self.ai_system = game_services.get_world().get_system_base::<AISystem>();
		self.spawner_system = game_services.get_world().get_system_base::<SpawnMobSystem>();
	}

	fn update<'sdl_all, 'l>(&mut self, _game_services: &mut GameServices<'sdl_all,'l>) -> bool {
		self.ai_system.as_ref().unwrap().upgrade().unwrap().read().unwrap().len_entities() > 0 ||
		self.spawner_system.as_ref().unwrap().upgrade().unwrap().read().unwrap().len_entities() > 0
	}
}

pub trait LevelPhaseBasic {
	fn base(&mut self) -> &mut dyn LevelPhase;
}

impl<T: LevelPhaseBasic> LevelPhase for T {
	fn on_enter<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>) {
		self.base().on_enter(game_services)
	}

	fn update<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>) -> bool {
		self.base().update(game_services)
	}
}
