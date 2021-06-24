use sdl2::{event::Event, keyboard::Keycode};
use tuple_list::tuple_list;
use crate::{core::{common::GameServices, ecs::SystemHolder, states::{self, State, StateWithSystems}}, systems::{animation::AnimationSystem, graphics::GraphicsSystem, input::InputSystem}};

pub struct PauseState {
	resume: bool
}

impl PauseState {
	pub fn new() -> Self {
		PauseState {
			resume: false
		}
	}
}

impl states::StateSystems for PauseState {
	type Systems = tuple_list!(GraphicsSystem, InputSystem, AnimationSystem);
}

impl State for PauseState {
	fn on_enter<'sdl_all, 'l>(&mut self, _systems: &mut SystemHolder, _game_services: &mut GameServices<'sdl_all,'l>, _create: bool) {
		println!("GAME PAUSED");
	}

	fn on_event(&mut self, event: &Event) -> bool {
		match event {
			Event::Quit { .. }
			| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
			} => return true,
			Event::KeyDown {
					keycode: Some(Keycode::Space),
					repeat: false,
					..
			} => {
				self.resume = true;
			}
			_ => {}
		}
		false
	}

	fn update<'sdl_all, 'l>(&mut self, _next_state: &mut Option<StateWithSystems>, _game_services: &mut GameServices<'sdl_all,'l>) -> bool {
		! self.resume
	}

	fn on_leave<'sdl_all, 'l>(&mut self, _game_services: &mut GameServices<'sdl_all,'l>, _destroy: bool) {
		println!("GAME RESUMED {}", _destroy);
	}
}
