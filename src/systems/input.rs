use std::{convert::TryFrom, sync::{Arc, RwLock}};

use tuple_list::tuple_list_type;

use crate::{components::{force::ForceComponent, input::{InputComponent, PlayerInput}, transform::TransformComponent}, core::{common::{self, GameServices}, ecs::{EntityId, Runnable, System, SystemComponents, SystemNewable}}, factory};


const PLAYER_SHOT_TIMER_MS: u64 = 300;
const SHOT_LIFETIME_MS: u64 = 2000;

pub struct InputSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for InputSystem {
	type Components = tuple_list_type!(TransformComponent, ForceComponent, InputComponent);
}

impl SystemNewable<InputSystem, ()> for InputSystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		InputSystem {
			base: base
		}
	}
}

impl InputSystem {
	fn shoot<'sdl_all, 'l>(entity_id: &EntityId, game_services: &mut GameServices<'sdl_all, 'l>) {
		let shot_width = 8 * 4;
		let shot_height = 13 * 4;
		let pos = game_services.get_world().get_component::<TransformComponent>(entity_id).unwrap();
		let shot_pos = (pos.x as i32 + shot_width as i32/ 2, pos.y as i32 - shot_height as i32 / 2);
		factory::create_shot("shot.png", shot_pos.0, shot_pos.1, shot_width, shot_height, 0.0, -10.0, SHOT_LIFETIME_MS, game_services);
	}
}

impl Runnable for InputSystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for entity in self.base.read().unwrap().iter_entities() {

			let (inputs_iter, power) = {
				let input = game_services.get_world().get_component::<InputComponent>(entity).unwrap();
				(input.inputs.iter().enumerate(), input.power)
			};

			let mut direction_x = 0.0;
			let mut shoot = false;
			for (i, vkey) in inputs_iter {
				if *vkey {
					match PlayerInput::try_from(&i).unwrap() {
						PlayerInput::LEFT => direction_x = -1.0,
						PlayerInput::RIGHT => direction_x = 1.0,
						PlayerInput::SHOOT => shoot = true,
						_ => {},
					}
				}
			}
			let force = game_services.get_world_mut().get_component_mut::<ForceComponent>(entity).unwrap();
			force.vx = power * direction_x;

			if shoot {
				let input = game_services.get_world_mut().get_component_mut::<InputComponent>(entity).unwrap();
				if (common::current_time_ms() - input.shot_timer_start) >= PLAYER_SHOT_TIMER_MS {
					input.shot_timer_start = common::current_time_ms();
					Self::shoot(entity, game_services);
				}
			}
		}
	}
}
