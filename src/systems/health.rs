use std::sync::{Arc, RwLock};

use tuple_list::tuple_list_type;

use crate::{components::health::HealthComponent, core::{common::{GameServices}, ecs::{Runnable, System, SystemComponents, SystemNewable}}};

pub struct HealthSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for HealthSystem {
	type Components = tuple_list_type!(HealthComponent);
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
				// TODO create an entity that plays a death animation / explosion at the location of the removed entity
				world.remove_entity(entity);
			}
		}
	}
}
