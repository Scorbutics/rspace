use std::sync::{Arc, RwLock};

use rand::Rng;
use tuple_list::tuple_list_type;

use crate::{components::{ai::AIComponent, hitbox::HitboxComponent, spawner::{SpawnerComponent, SpawnerType}, transform::TransformComponent}, core::{common::{self, GameServices}, ecs::{Runnable, System, SystemComponents, SystemNewable}}, factory, levels::phases::{TrajectoryGenerator, TrajectoryType}};

pub struct SpawnMobSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for SpawnMobSystem {
	type Components = tuple_list_type!(SpawnerComponent, TransformComponent, HitboxComponent);
}

impl SystemNewable<SpawnMobSystem, ()> for SpawnMobSystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		SpawnMobSystem {
			base: base
		}
	}
}

impl SpawnMobSystem {
	fn spawn_enemies<'sdl_all, 'l>(game_services: &mut GameServices<'sdl_all, 'l>, origin_x: i32, origin_y: i32, angle_radian: f32, pos_offset_x: i32, pos_offset_y: i32, speed: f32, number: u16, luck_percents: f32, trajectory: TrajectoryType) {
		//let mut before: Option<EntityId> = None;
		for index in 0..number {
			let mut rng = rand::thread_rng();
			let random_percent= rng.gen_range(0.0, 100.0) as f32;
			if random_percent < luck_percents {
				let direction_vector = (f32::cos(index as f32 * angle_radian), f32::sin(index as f32 * angle_radian));
				let position = (origin_x as f32 + direction_vector.0 * pos_offset_x as f32, origin_y as f32 + direction_vector.1 * pos_offset_y as f32);
				let width = 16 * 4;
				let height = 16 * 4;
				let enemy = factory::create_living_entity("enemy_spaceship.png", position.0 as i32, position.1 as i32, width, height, game_services);
				let hitbox = game_services.get_world_mut().get_component_mut::<HitboxComponent>(&enemy).unwrap();
				// Real enemy hitbox has an offset from the graphical one in order to make the shot "feels" like it really landed on the enemy
				hitbox.hitbox.h -= 5 * 4;
				let mut ai = AIComponent::new();
				ai.set_movement_patterns(TrajectoryGenerator::generate_enemy_movement_pattern(&trajectory, common::current_time_ms() + (index as f32 * 300.0 / speed) as u64,
				((game_services.draw_context.screen_width() / 2) as f32, (game_services.draw_context.screen_height() / 2) as f32), game_services.draw_context.screen_width(), game_services.draw_context.screen_height()));
				game_services.get_world_mut().add_component::<AIComponent>(&enemy, ai);
			}
		}
	}
}

impl Runnable for SpawnMobSystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for entity in self.base.read().unwrap().iter_entities() {
			let world = game_services.get_world_mut();
			let spawner = world.get_component::<SpawnerComponent>(entity).unwrap();
			if spawner.num > 0 && common::current_time_ms() - spawner.last_spawn_ms > spawner.frequency_ms as u64 {
				let area = world.get_component::<HitboxComponent>(entity).unwrap();
				let pos = (area.hitbox.width() as i32/ spawner.num as i32, area.hitbox.height() as i32/ spawner.num as i32);
				let origin = world.get_component::<TransformComponent>(entity).unwrap();
				let origin = (origin.x as i32 - (area.hitbox.width() as i32/2) + area.hitbox.x, origin.y as i32 - (area.hitbox.height() as i32/2) + area.hitbox.y);
				let propulsion = spawner.propulsion;
				let num = spawner.num;
				let luck_percents = spawner.luck_percents;
				let trajectory = spawner.trajectory_type.clone();
				match spawner.spawner_type {
					SpawnerType::CIRCLE => {
						let angle_radian = spawner.max_angle / spawner.num as f32;
						Self::spawn_enemies(game_services, origin.0, origin.1,
							angle_radian, pos.0, pos.1, propulsion, num, luck_percents, trajectory);
					},
					SpawnerType::POINT => {
						Self::spawn_enemies(game_services, origin.0, origin.1,
							0.0, pos.0, pos.1, propulsion, num, luck_percents, trajectory);
					},
					SpawnerType::LINEAR => todo!(),
				}
				let spawner = game_services.get_world_mut().get_component_mut::<SpawnerComponent>(entity).unwrap();
				spawner.last_spawn_ms = common::current_time_ms();
				if spawner.randomize_pos {
					let spawn_pos = factory::random_outside_spawn_pos(game_services.draw_context.screen_width(), game_services.draw_context.screen_height());
					let pos = game_services.get_world_mut().get_component_mut::<TransformComponent>(entity).unwrap();
					pos.x = spawn_pos.0;
					pos.y = spawn_pos.1;
				}

				let spawner = game_services.get_world_mut().get_component_mut::<SpawnerComponent>(entity).unwrap();
				if spawner.countdown >= 1 {
					spawner.countdown -= 1;
				} else {
					game_services.get_world_mut().remove_entity(entity);
				}
			}
		}
	}
}
