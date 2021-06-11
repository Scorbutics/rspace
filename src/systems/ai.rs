use std::sync::{Arc, RwLock};

use tuple_list::tuple_list_type;

use crate::{components::{ai::AIComponent, force::ForceComponent, transform::TransformComponent}, core::{common::{self, GameServices}, ecs::{Runnable, System, SystemComponents, SystemNewable}}, maths};

pub struct AISystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for AISystem {
	type Components = tuple_list_type!(AIComponent, ForceComponent, TransformComponent);
}

impl SystemNewable<AISystem, ()> for AISystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		AISystem {
			base: base
		}
	}
}

impl Runnable for AISystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for entity_id in self.base.read().unwrap().iter_entities() {
			let ai = game_services.get_world().get_component::<AIComponent>(entity_id).unwrap();
			let current_pos = game_services.get_world().get_component::<TransformComponent>(entity_id).unwrap();
			let current_pos = (current_pos.x, current_pos.y);
			let target_pos = (230.0, 230.0);
			let distance = f32::sqrt(maths::distance_squared(current_pos, target_pos));
			let next_pos = ai.next_position((target_pos.0 - current_pos.0, target_pos.1 - current_pos.1));
			let power = 100.0;
			if next_pos.is_some() {
				let force = game_services.get_world_mut().get_component_mut::<ForceComponent>(entity_id).unwrap();
				force.ax = next_pos.unwrap().0 * (power / distance);
				force.ay = next_pos.unwrap().1 * (power / distance);

				//println!("LOL : {}, {}", force.vx, force.vy);
			}
		}
	}
}
