use std::sync::{Arc, RwLock};

use tuple_list::tuple_list_type;

use crate::{components::{ai::AIComponent, animation::AnimationComponent, input::{InputComponent, State}, sprite::SpriteComponent}, core::{common::{GameServices}, ecs::{Runnable, System, SystemComponents, SystemNewable}}};

pub struct AnimationSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for AnimationSystem {
	type Components = tuple_list_type!(AnimationComponent, SpriteComponent);
}

impl SystemNewable<AnimationSystem, ()> for AnimationSystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		AnimationSystem {
			base: base
		}
	}
}

impl AnimationSystem {
	fn compute_animation_depending_on_moving(last_state: State, state: State) -> Option<usize> {
		if state == last_state {
			return None;
		}

		let next;
		if last_state == State::Stand {
			next = if state == State::MoveRight { 2 } else { 0 };
		} else {
			if state == State::Stand {
				next = if last_state == State::MoveRight { 3 } else { 1 };
			} else {
				next = if last_state == State::MoveRight { 5 } else { 4 };
			}
		}

		Some(next)
	}
}

impl Runnable for AnimationSystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for entity_id in self.base.read().unwrap().iter_entities() {

			if game_services.get_world().has_component::<InputComponent>(entity_id) {
				let input = game_services.get_world().get_component::<InputComponent>(entity_id).unwrap();
				let (last_state, state) = (input.last_state, input.state);
				let next = Self::compute_animation_depending_on_moving(last_state, state);
				if next.is_some() {
					let animation = game_services.get_world_mut().get_component_mut::<AnimationComponent>(entity_id).unwrap();
					animation.next(next.unwrap());
				}
			}

			if game_services.get_world().has_component::<AIComponent>(entity_id) {
				let ai = game_services.get_world().get_component::<AIComponent>(entity_id).unwrap();
				let (last_state, state) = (ai.last_state, ai.state);
				let next = Self::compute_animation_depending_on_moving(last_state, state);
				if next.is_some() {
					let animation = game_services.get_world_mut().get_component_mut::<AnimationComponent>(entity_id).unwrap();
					animation.next(next.unwrap());
				}
			}

			let animation = game_services.get_world_mut().get_component_mut::<AnimationComponent>(entity_id).unwrap();
			animation.update();

			let frame = animation.get_offset();
			let origin = animation.get_origin();
			let sprite = game_services.get_world_mut().get_component_mut::<SpriteComponent>(entity_id).unwrap();
			sprite.spritesheet_index.0 = frame;
			sprite.spritesheet_index.1 = origin;
		}
	}
}
