use std::sync::{Arc, RwLock};

use tuple_list::tuple_list_type;

use crate::{components::{animation::AnimationComponent, input::{InputComponent, State}, sprite::SpriteComponent}, core::{common::{GameServices}, ecs::{Runnable, System, SystemComponents, SystemNewable}}};

pub struct AnimationSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for AnimationSystem {
	type Components = tuple_list_type!(InputComponent, AnimationComponent, SpriteComponent);
}

impl SystemNewable<AnimationSystem, ()> for AnimationSystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		AnimationSystem {
			base: base
		}
	}
}

impl AnimationSystem {
	fn change_animation_depending_on_moving(all: &mut AnimationComponent, last_state: State, state: State) -> bool {
		if state == last_state {
			return false;
		}

		let current;
		if last_state == State::Stand {
			current = if state == State::MoveRight { 0 } else { 2 };
		} else {
			if state == State::Stand {
				current = if last_state == State::MoveRight { 3 } else { 1 };
			} else {
				current = if last_state == State::MoveRight { 5 } else { 4 };
			}
		}

		let last_offset = all.get_offset();
		//TODO for more cohence we should enqueue instead of replace
		all.current = current;
		all.reset();
		all.current_offset(last_offset);
		all.start();

		true
	}
}

impl Runnable for AnimationSystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for entity_id in self.base.read().unwrap().iter_entities() {
			let input = game_services.get_world().get_component::<InputComponent>(entity_id).unwrap();
			let (last_state, state) = (input.last_state, input.state);
			let animation = game_services.get_world_mut().get_component_mut::<AnimationComponent>(entity_id).unwrap();
			Self::change_animation_depending_on_moving(animation, last_state, state);
			animation.update();

			let frame = animation.get_offset();
			let origin = animation.get_origin();
			let sprite = game_services.get_world_mut().get_component_mut::<SpriteComponent>(entity_id).unwrap();
			sprite.spritesheet_index.0 = frame;
			sprite.spritesheet_index.1 = origin;
		}
	}
}
