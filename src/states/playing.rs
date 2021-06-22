use sdl2::{event::Event, keyboard::Keycode};

use crate::{components::{hitbox::HitboxComponent, input::{InputComponent, PlayerInput}}, core::{common::GameServices, ecs, states}, factory, levels::{level::Level, level1::{Level1Mid, Level1Start}}};

use super::{background::BackgroundStarField, pause::PauseState};

pub struct PlayingState {
	player: ecs::EntityId,
	pause: bool,
	inputs: [bool; PlayerInput::LAST as usize],
	levels: Vec<Level>,
	current_level_index: usize,
	background: Option<BackgroundStarField>
}

impl PlayingState {
	pub fn new() -> Self {
		PlayingState {
			player: 0,
			pause: false,
			inputs: [false; PlayerInput::LAST as usize],
			levels: Vec::new(),
			current_level_index: 0,
			background: None
		}
	}
}

impl states::State for PlayingState {
	fn on_enter<'sdl_all, 'l>(&mut self, game_services: & mut GameServices<'sdl_all, 'l>, create: bool) {
		println!("ENTER PLAYING ! {}", create);
		if create {
			self.background = Some(BackgroundStarField::new(game_services));
			let src_width = 16;
			let src_height = 16;
			let width = src_width * 4;
			let height = src_height * 4;
			let x = ((game_services.draw_context.screen_width() - width) / 2)  as i32;
			let y = (game_services.draw_context.screen_height() - height - 5) as i32;
			self.player = factory::create_player("spaceship.png", x, y, 0, width, height, 10.0, game_services);
			let hitbox = game_services.get_world_mut().get_component_mut::<HitboxComponent>(&self.player).unwrap();
			hitbox.hitbox.w /= 2;
			hitbox.hitbox.h /= 2;
			hitbox.hitbox.x += hitbox.hitbox.w / 2;
			hitbox.hitbox.y += hitbox.hitbox.h / 2;

			self.levels.push(Level::new(vec![Box::new(Level1Start::new()), Box::new(Level1Mid::new()), Box::new(Level1Start::new())]));
		}
	}

	fn update<'sdl_all, 'l>(&mut self, next_state: &mut Option<Box<dyn states::State>>, game_services: &mut GameServices<'sdl_all,'l>) -> bool {
		if self.pause {
			let pause_state: Option<Box<dyn states::State>> = Some(Box::new(PauseState::new()));
			*next_state = pause_state;
			self.pause = false;
		}
		let game_continue = game_services.get_world().is_alive(&self.player);
		if game_continue {
			let input = game_services.get_world_mut().get_component_mut::<InputComponent>(&self.player).unwrap();
			input.inputs = self.inputs.clone();
		}

		self.background.as_mut().unwrap().update(game_services);

		if self.current_level_index < self.levels.len() {
			if ! self.levels[self.current_level_index].update(game_services) {
				self.current_level_index += 1;
			}
			game_continue
		} else {
			false
		}
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
					keycode: Some(Keycode::Q),
					..
				} => {
					self.inputs[PlayerInput::LEFT as usize] = true;
				},
				Event::KeyDown {
					keycode: Some(Keycode::D),
					..
				} => {
					self.inputs[PlayerInput::RIGHT as usize] = true;
				},
				Event::KeyDown {
					keycode: Some(Keycode::Z),
					..
				} => {
					self.inputs[PlayerInput::UP as usize] = true;
				},
				Event::KeyDown {
					keycode: Some(Keycode::S),
					..
				} => {
					self.inputs[PlayerInput::DOWN as usize] = true;
				},
				Event::KeyDown {
					keycode: Some(Keycode::Space),
					repeat: false,
					..
				} => {
					self.inputs[PlayerInput::SHOOT as usize] = true;
				},
				Event::KeyUp {
					keycode: Some(Keycode::Q),
					..
				} => {
					self.inputs[PlayerInput::LEFT as usize] = false;
				},
				Event::KeyUp {
					keycode: Some(Keycode::D),
					..
				} => {
					self.inputs[PlayerInput::RIGHT as usize] = false;
				},
				Event::KeyUp {
					keycode: Some(Keycode::Z),
					..
				} => {
					self.inputs[PlayerInput::UP as usize] = false;
				},
				Event::KeyUp {
					keycode: Some(Keycode::S),
					..
				} => {
					self.inputs[PlayerInput::DOWN as usize] = false;
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
