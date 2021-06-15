use std::sync::{Arc, RwLock};

use tuple_list::tuple_list_type;

use crate::{components::{health::HealthComponent, hitbox::HitboxComponent, lifetime::LifetimeComponent, sprite::SpritesheetOrientation, transform::TransformComponent}, core::{common::{self, GameServices}, ecs::{Runnable, System, SystemComponents, SystemNewable}}, factory, maths};

pub struct HealthSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for HealthSystem {
	type Components = tuple_list_type!(HealthComponent, TransformComponent, HitboxComponent);
}

impl SystemNewable<HealthSystem, ()> for HealthSystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		HealthSystem {
			base: base
		}
	}
}

impl Runnable for HealthSystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for entity in self.base.read().unwrap().iter_entities() {
			let world = game_services.get_world_mut();
			let health = world.get_component::<HealthComponent>(entity).unwrap();
			if health.health_points <= 0 {
				world.remove_entity(entity);
				let position = maths::center(world, entity);
				let explosion = factory::create_animation("explosion.png", position.0 as i32 - 16, position.1 as i32 - 16, 16, 16, 16*2, 16*2, 8, SpritesheetOrientation::HORIZONTAL, 30, game_services);
				game_services.get_world_mut().add_component::<LifetimeComponent>(&explosion, LifetimeComponent::new(common::current_time_ms() + 300));
			}
		}
	}
}
