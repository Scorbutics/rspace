use std::sync::{Arc, RwLock};

use tuple_list::tuple_list_type;

use crate::{components::{ai::AIComponent, force::ForceComponent, hitbox::HitboxComponent, transform::TransformComponent}, core::{common::{GameServices}, ecs::{Runnable, System, SystemComponents, SystemNewable}}, maths};

pub struct AISystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for AISystem {
	type Components = tuple_list_type!(AIComponent, ForceComponent, TransformComponent, HitboxComponent);
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
			let current_pos = maths::center(game_services.get_world(), entity_id);

			let ai = game_services.get_world_mut().get_component_mut::<AIComponent>(entity_id).unwrap();

			// TODO ai.speed ?
			let power = 3.0;
			let next_pos = ai.next_position(&current_pos, &power);
			if next_pos.is_some() {
				let force = game_services.get_world_mut().get_component_mut::<ForceComponent>(entity_id).unwrap();
				let velocity_vector = maths::next_step_to_pos(current_pos, next_pos.unwrap(), power);
				force.vx = velocity_vector.0;
				force.vy = velocity_vector.1;
			} else {
				game_services.get_world_mut().remove_entity(entity_id);
				println!("IA DEAD : {}", *entity_id);
			}
		}
	}
}
