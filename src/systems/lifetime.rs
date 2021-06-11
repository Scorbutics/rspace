use std::sync::{Arc, RwLock};

use tuple_list::tuple_list_type;

use crate::{components::lifetime::LifetimeComponent, core::{common::{self, GameServices}, ecs::{Runnable, System, SystemComponents, SystemNewable}}};

pub struct LifetimeSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for LifetimeSystem {
	type Components = tuple_list_type!(LifetimeComponent);
}

impl SystemNewable<LifetimeSystem, ()> for LifetimeSystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		LifetimeSystem {
			base: base
		}
	}
}

impl Runnable for LifetimeSystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for entity in self.base.read().unwrap().iter_entities() {
			let world = game_services.get_world_mut();
			let shot = world.get_component::<LifetimeComponent>(entity).unwrap();
			if common::current_time_ms() > shot.life_timer_end {
				world.remove_entity(entity);
			}
		}
	}
}
