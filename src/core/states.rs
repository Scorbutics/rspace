use crate::sdl2;
use sdl2::event::Event;

use super::common::GameServices;

pub trait State {
	fn on_enter<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>, create: bool);
	fn on_event(&mut self, event: &Event) -> bool;
	fn update<'sdl_all, 'l>(&mut self, next_state: &mut Option<Box<dyn State>>, game_services: &mut GameServices<'sdl_all,'l>) -> bool;
	fn on_leave<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>, destroy: bool);
}

pub struct StateDispatcher {
	states: Vec<Box<dyn State>>,
	next_state: Option<Box<dyn State>>
}

impl StateDispatcher {
	pub fn new() -> Self {
		StateDispatcher {
			states: Vec::new(),
			next_state: Option::None
		}
	}

	pub fn enqueue_state(&mut self, next_state: Box<dyn State>) {
		self.next_state = Some(next_state);
	}

	pub fn update<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>) -> bool {
		let next_state = self.next_state.take();
		if next_state.is_some() {
			let old_state = self.states.last_mut();
			match old_state {
				Some(v) => v.on_leave(game_services, false),
				None => {},
			}
			let mut current_state = next_state.unwrap();
			current_state.on_enter(game_services, true);
			self.states.push(current_state);
		}

		let mut current_state = self.states.last_mut();
		let no_more_states = current_state.is_none();
		if ! no_more_states {
			let state_continue = current_state.as_mut().unwrap().update(&mut self.next_state, game_services);
			if ! state_continue {
				self.states.pop().unwrap().on_leave(game_services, true);
				let current_state = self.states.last_mut();
				if current_state.is_some() {
					current_state.unwrap().on_enter(game_services, false);
				}
			}
		}
		! no_more_states
	}

	pub fn dispatch_event(&mut self, event: &Event) -> bool {
		if self.states.last_mut().is_some() {
			self.states.last_mut().unwrap().on_event(event)
		} else {
			false
		}
	}
}
