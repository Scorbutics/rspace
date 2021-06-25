use std::sync::{atomic::AtomicUsize};

use crate::sdl2;
use fixedbitset::FixedBitSet;
use once_cell::sync::OnceCell;
use sdl2::event::Event;
use tuple_list::TupleList;

use super::{common::GameServices, ecs::{SystemHolder, WeakRunnable}, meta::{self, IdCounter, TypeMaskSetBit}};

pub trait State {
	fn on_enter<'sdl_all, 'l>(&mut self, runnables: &mut Vec<WeakRunnable>, game_services: &mut GameServices<'sdl_all,'l>, create: bool, last_state_id: Option<usize>);
	fn on_event(&mut self, event: &Event) -> bool;
	fn update<'sdl_all, 'l>(&mut self, next_state: &mut Option<StateWithSystems>, game_services: &mut GameServices<'sdl_all,'l>) -> bool;
	fn on_leave<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>, destroy: bool);
}

const STATES_MAX_SYSTEMS: usize = 100;

pub trait StateSystems : State {
	type Systems: TupleList + TypeMaskSetBit;
}

pub struct StateWithSystems {
	state: Box<dyn State>,
	mask: FixedBitSet,
	id: usize
}

pub static SYSTEM_ID_COUNTER: IdCounter = IdCounter { cell: OnceCell::new(), atomic: AtomicUsize::new(0) };
pub static STATE_ID_COUNTER: IdCounter = IdCounter { cell: OnceCell::new(), atomic: AtomicUsize::new(0) };

impl StateWithSystems {
	pub fn new<T: State + StateSystems + 'static>(state: Box<T>) -> Self {
		let mut mask = FixedBitSet::with_capacity(STATES_MAX_SYSTEMS);
		T::Systems::set_bitset(&SYSTEM_ID_COUNTER, &mut mask);
		let id = meta::numeric_type_id::<T>(&STATE_ID_COUNTER);
		StateWithSystems {
			state: state,
			mask: mask,
			id: *id
		}
	}
}

pub struct StateDispatcher {
	states: Vec<Option<StateWithSystems>>,
	states_ids: Vec<usize>,
	next_state: Option<StateWithSystems>
}

impl StateDispatcher {
	pub fn new() -> Self {
		let mut states = Vec::with_capacity(STATES_MAX_SYSTEMS);
		states.resize_with(STATES_MAX_SYSTEMS, || Option::None);
		StateDispatcher {
			states: states,
			states_ids: Vec::new(),
			next_state: Option::None
		}
	}

	pub fn enqueue_state<T: State + StateSystems + 'static> (&mut self, next_state: Box<T>) {
		self.next_state = Some(StateWithSystems::new(next_state));
	}

	fn top_state_mut(&mut self) -> Option<&mut StateWithSystems> {
		if let Some(last) = self.states_ids.last() {
			self.states[*last].as_mut()
		} else {
			None
		}
	}

	fn top_state_id(&self) -> Option<&usize> {
		self.states_ids.last()
	}

	fn top_previous_state_id(&self) -> Option<&usize> {
		if self.states_ids.len() < 2 {
			None
		} else {
			self.states_ids.get(self.states_ids.len() - 2)
		}
	}

	fn pop_state(&mut self) -> Option<StateWithSystems> {
		if let Some(last) = self.states_ids.pop() {
			self.states[last].take()
		} else {
			None
		}
	}

	fn pause_current_state<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>, destroy: bool) -> Option<usize> {
		if let Some(state) = self.top_state_mut() {
			state.state.on_leave(game_services, destroy);
			Some(*self.top_state_id().unwrap())
		} else {
			None
		}
	}

	fn push_state<'sdl_all, 'l>(&mut self, state: StateWithSystems) {
		assert!(self.states_ids.len() < STATES_MAX_SYSTEMS);
		let id =  state.id;
		self.states_ids.push(id);
		println!("PUSHING STATE {} (total states {})", id, self.states_ids.len());
		self.states[id] = Some(state);
	}

	fn refresh_systems<'sdl_all, 'l>(&mut self, systems: &mut SystemHolder, game_services: &mut GameServices<'sdl_all,'l>, last_state_id: Option<&usize>, next_state_id: usize) {
		let next_mask = &self.states[next_state_id].as_ref().unwrap().mask;
		if last_state_id.is_some() {
			let last_mask = &self.states[*last_state_id.unwrap()].as_ref().unwrap().mask;
			let enable_diff_mask = next_mask.difference(last_mask);
			let disable_diff_mask = last_mask.difference(&next_mask);

			for system_id in enable_diff_mask {
				println!("ACTIVATING SYSTEM {}", system_id);
				systems.enable_system(game_services.get_world_mut(), system_id as u64);
			}

			for system_id in disable_diff_mask {
				println!("DEACTIVATING SYSTEM {}", system_id);
				systems.disable_system(game_services.get_world_mut(), system_id as u64);
			}
		} else {
			for system_id in next_mask.difference(&FixedBitSet::with_capacity(STATES_MAX_SYSTEMS)) {
				println!("ACTIVATING SYSTEM {}", system_id);
				systems.enable_system(game_services.get_world_mut(), system_id as u64);
			}
		}
	}

	fn resume_state<'sdl_all, 'l>(&mut self, runnables: &mut Vec<WeakRunnable>, game_services: &mut GameServices<'sdl_all,'l>, create: bool, last_state_id: Option<usize>) {
		println!("RESUMING STATE (total states {})", self.states_ids.len());
		if let Some(next_state) = self.top_state_mut() {
			next_state.state.on_enter(runnables, game_services, create, last_state_id);
		}
	}

	pub fn update<'sdl_all, 'l>(&mut self, systems: &mut SystemHolder, runnables: &mut Vec<WeakRunnable>, game_services: &mut GameServices<'sdl_all,'l>) -> bool {
		if let Some(next_state) = self.next_state.take() {
			let last_state_id = self.pause_current_state(game_services, false);
			self.push_state(next_state);
			self.refresh_systems(systems, game_services, last_state_id.as_ref(), *self.top_state_id().unwrap());
			self.resume_state(runnables, game_services, true, last_state_id);
		}

		if let Some(last_id) = self.states_ids.last() {
			let current_state = self.states[*last_id].as_mut().unwrap();
			let n = &mut self.next_state;
			let state_continue = current_state.state.update(n, game_services);
			if ! state_continue {
				let last_state_id = self.pause_current_state(game_services, true);
				let mut no_more_state = true;
				if let Some(next_state_id)  = self.top_previous_state_id() {
					let n = *next_state_id;
					no_more_state = false;
					self.refresh_systems(systems, game_services, last_state_id.as_ref(), n);
				}
				self.pop_state();
				self.resume_state(runnables, game_services, false, last_state_id);
				! no_more_state
			} else {
				true
			}
		} else {
			false
		}
	}

	pub fn dispatch_event(&mut self, event: &Event) -> bool {
		if let Some(current_state) = self.top_state_mut() {
			current_state.state.on_event(event)
		} else {
			false
		}
	}
}
