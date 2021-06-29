use std::sync::{Arc, RwLock, Weak};

use tuple_list::tuple_list_type;

use crate::{components::{health::HealthComponent, hitbox::HitboxComponent, input::InputComponent, shot::{ShotComponent, ShotType}, transform::TransformComponent}, core::{common::{GameServices}, ecs::{EntityId, Runnable, System, SystemComponents, SystemNewable, World}}, maths};

use super::health::HealthSystem;

pub struct ShotSystem {
	base: Arc<RwLock<System>>,
	health_system: Option<Weak<RwLock<System>>>
}

impl SystemComponents for ShotSystem {
	type Components = tuple_list_type!(ShotComponent, HitboxComponent, TransformComponent);
}

impl SystemNewable<ShotSystem, ()> for ShotSystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		ShotSystem {
			base: base,
			health_system: None
		}
	}
}

impl ShotSystem {
	fn check_collision(world: &World, shot: &EntityId, target: &EntityId) -> Option<EntityId> {
		if maths::collision(world, shot, target) {
			Some(*target)
		} else {
			None
		}
	}
}

impl Runnable for ShotSystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		if self.health_system.is_none() {
			self.health_system = game_services.get_world().get_system_base::<HealthSystem>();
		}
		for entity in self.base.read().unwrap().iter_entities() {

			// (For this game we can code a naive collision system that will iterate all eligible entities except self and check collisions with it)
			for health_entity in self.health_system.as_ref().unwrap().upgrade().unwrap().read().unwrap().iter_entities() {
				if health_entity != entity {
					let world = game_services.get_world_mut();
					let shot = world.get_component::<ShotComponent>(entity).unwrap();
					let damages = shot.damages;
					let target_entity: Option<EntityId> = match shot.shot_type {
						ShotType::PLAYER => {
							// No input => This is an enemy
							if ! world.has_component::<InputComponent>(health_entity) {
								Self::check_collision(world, entity, health_entity)
							} else {
								None
							}
						},
						ShotType::ENEMY => {
							// Input => This is the player
							if world.has_component::<InputComponent>(health_entity) {
								Self::check_collision(world, entity, health_entity)
							} else {
								None
							}
						}
					};

					if target_entity.is_some() {
						//println!("DAMAGE !");
						world.get_component_mut::<HealthComponent>(&target_entity.unwrap()).unwrap().health_points -= damages as i64;
						world.remove_entity(entity);
						break;
					}
				}
			}
		}
	}
}
