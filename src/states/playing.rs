use sdl2::{event::Event, keyboard::Keycode};

use crate::{components::{input::{InputComponent, PlayerInput}, lifetime::LifetimeComponent, spawner::{SpawnerComponent, SpawnerType}}, core::{common::{self, GameServices}, ecs, states}, factory};

use super::pause::PauseState;

pub struct PlayingState {
	player: ecs::EntityId,
	pause: bool,
	inputs: [bool; PlayerInput::LAST as usize]
}

impl PlayingState {
	pub fn new() -> Self {
		PlayingState {
			player: 0,
			pause: false,
			inputs: [false; PlayerInput::LAST as usize]
		}
	}
}

impl states::State for PlayingState {
	fn on_enter<'sdl_all, 'l>(&mut self, game_services: & mut GameServices<'sdl_all, 'l>, create: bool) {
		println!("ENTER PLAYING ! {}", create);
		if create {
			let width = 16 * 4;
			let height = 16 * 4;
			let x = ((game_services.draw_context.screen_width() - width)/2)  as i32;
			let y = (game_services.draw_context.screen_height() - height - 5) as i32;
			self.player = factory::create_player("spaceship.png", x, y, width, height, 10.0, game_services);

			let spawn_pos = ((game_services.draw_context.screen_width() / 2) as i32, (game_services.draw_context.screen_height() / 2) as i32);
			let spawner = factory::create_entity("",  spawn_pos.0, spawn_pos.1, width, height, game_services);
			game_services.get_world_mut().add_component::<SpawnerComponent>(&spawner, SpawnerComponent::new(SpawnerType::CIRCLE, 3000, 50.0, 3, 1.0, std::f32::consts::PI * 2.0));
			game_services.get_world_mut().add_component::<LifetimeComponent>(&spawner, LifetimeComponent::new(common::current_time_ms() + 5000));
		}
	}

	fn update<'sdl_all, 'l>(&mut self, next_state: &mut Option<Box<dyn states::State>>, game_services: &mut GameServices<'sdl_all,'l>) -> bool {
		if self.pause {
			let pause_state: Option<Box<dyn states::State>> = Some(Box::new(PauseState::new()));
			*next_state = pause_state;
			self.pause = false;
		}
		let input = game_services.get_world_mut().get_component_mut::<InputComponent>(&self.player).unwrap();
		input.inputs = self.inputs.clone();
		true
	}

	fn on_leave<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>, destroy: bool) {
		println!("LEAVE PLAYING ! {}", destroy);
		if destroy {
			game_services.get_world_mut().remove_entity(&self.player);
		}
	}

	fn on_event(&mut self, event: &Event) -> bool {
		match event {
				Event::Quit { .. }
				| Event::KeyDown {
						keycode: Some(Keycode::Escape),
						..
				} => return true,
				Event::KeyDown {
						keycode: Some(Keycode::Return),
						repeat: false,
						..
				} => {
					self.pause = true;
				},
				Event::KeyDown {
					keycode: Some(Keycode::Left),
					..
				} => {
					self.inputs[PlayerInput::LEFT as usize] = true;
				},
				Event::KeyDown {
					keycode: Some(Keycode::Right),
					..
				} => {
					self.inputs[PlayerInput::RIGHT as usize] = true;
				},
				Event::KeyDown {
					keycode: Some(Keycode::Space),
					repeat: false,
					..
				} => {
					self.inputs[PlayerInput::SHOOT as usize] = true;
				},
				Event::KeyUp {
					keycode: Some(Keycode::Left),
					..
				} => {
					self.inputs[PlayerInput::LEFT as usize] = false;
				},
				Event::KeyUp {
					keycode: Some(Keycode::Right),
					..
				} => {
					self.inputs[PlayerInput::RIGHT as usize] = false;
				},
				Event::KeyUp {
					keycode: Some(Keycode::Space),
					repeat: false,
					..
				} => {
					self.inputs[PlayerInput::SHOOT as usize] = false;
				},
				_ => {}
		}
		false
	}
}
