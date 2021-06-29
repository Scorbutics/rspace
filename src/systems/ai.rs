use std::sync::{Arc, RwLock, Weak};

use rand::Rng;
use tuple_list::tuple_list_type;

use crate::{components::{ai::AIComponent, force::ForceComponent, hitbox::HitboxComponent, input::State, shot::ShotType, sprite::SpriteComponent, transform::TransformComponent}, core::{common::{GameServices}, ecs::{EntityId, Runnable, System, SystemComponents, SystemNewable}}, factory, maths};

use super::input::InputSystem;

pub struct AISystem {
	base: Arc<RwLock<System>>,
	input_system: Option<Weak<RwLock<System>>>
}

impl SystemComponents for AISystem {
	type Components = tuple_list_type!(AIComponent, ForceComponent, TransformComponent, HitboxComponent);
}

impl SystemNewable<AISystem, ()> for AISystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		AISystem {
			base: base,
			input_system: None
		}
	}
}

const SHOT_LIFETIME_MS: u64 = 6000;

impl AISystem {
	fn shoot<'sdl_all, 'l>(entity_id: &EntityId, power: f32, input_system: &Weak<RwLock<System>>, game_services: &mut GameServices<'sdl_all, 'l>) {
		let shot_width = 16;
		let shot_height = 16 * 2;
		let pos = game_services.get_world().get_component::<TransformComponent>(entity_id).unwrap();
		let graphic_box = game_services.get_world().get_component::<SpriteComponent>(entity_id).unwrap().graphic_box;
		let shot_pos = (pos.x as i32 + graphic_box.w / 2 + graphic_box.x - shot_width / 2, pos.y as i32 + graphic_box.h + graphic_box.y);
		let input_entities_num = input_system.upgrade().unwrap().read().unwrap().len_entities();
		if input_entities_num > 0 {
			let mut rng = rand::thread_rng();
			let mut random_target_entity: i16 = rng.gen_range(0, input_entities_num) as i16;

			let mut target = 0;
			for target_id in input_system.upgrade().unwrap().read().unwrap().iter_entities() {
				if random_target_entity == 0 {
					target = *target_id;
					break;
				} else {
					random_target_entity -= 1;
				}
			}

			let entity_center = maths::center(game_services.get_world(), entity_id);
			let target_center = maths::center(game_services.get_world(), &target);

			let velocity = maths::next_step_to_pos(entity_center, target_center, power);
			factory::create_shot("shot.png", shot_pos.0, shot_pos.1, 1, shot_width as u32, shot_height as u32, velocity.0, velocity.1, SHOT_LIFETIME_MS, ShotType::ENEMY, game_services);
		}
	}

	fn compute_next_state(velocity_vector: &(f32, f32), trigger: f32) -> State {
		let v = f32::abs(velocity_vector.0);
		if v > - trigger && v < trigger {
			State::Stand
		} else if velocity_vector.0 > 0.0 {
			State::MoveRight
		} else {
			State::MoveLeft
		}
	}
}

impl Runnable for AISystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		if self.input_system.is_none() {
			self.input_system = game_services.get_world_mut().get_system_base::<InputSystem>();
		}
		for entity_id in self.base.read().unwrap().iter_entities() {
			let current_pos = maths::center(game_services.get_world(), entity_id);

			if game_services.get_world_mut().get_component_mut::<AIComponent>(entity_id).unwrap().can_shoot() {
				let shot_power = game_services.get_world_mut().get_component_mut::<AIComponent>(entity_id).unwrap().shot_power;
				Self::shoot(entity_id, shot_power, self.input_system.as_ref().unwrap(), game_services);
			}

			let ai = game_services.get_world_mut().get_component_mut::<AIComponent>(entity_id).unwrap();
			let power = ai.speed;
			let next_pos = ai.next_position(&current_pos, &power);
			if next_pos.is_some() {
				let force = game_services.get_world_mut().get_component_mut::<ForceComponent>(entity_id).unwrap();
				let velocity_vector = maths::next_step_to_pos(current_pos, next_pos.unwrap(), power);
				let state = Self::compute_next_state(&velocity_vector, 2.0);

				force.vx = velocity_vector.0;
				force.vy = velocity_vector.1;

				let ai = game_services.get_world_mut().get_component_mut::<AIComponent>(entity_id).unwrap();
				ai.last_state = ai.state;
				ai.state = state;
			} else {
				game_services.get_world_mut().remove_entity(entity_id);
				println!("IA DEAD : {}", *entity_id);
			}
		}
	}
}
