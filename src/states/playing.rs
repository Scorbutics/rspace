use std::sync::{Arc, RwLock};

use sdl2::{event::Event, keyboard::Keycode};
use tuple_list::tuple_list;

use crate::{components::{hitbox::HitboxComponent, input::{InputComponent, PlayerInput}}, core::{common::GameServices, ecs::{self, SystemHolder}, states::{self, StateWithSystems}}, factory, levels::{level::Level, level1::{Level1End, Level1Mid, Level1Mid2, Level1Start}, phase_basic_spawn::LevelPhaseBasicSpawn}, systems::{ai::AISystem, animation::AnimationSystem, followme::FollowMeSystem, graphics::GraphicsSystem, health::HealthSystem, input::InputSystem, lifetime::LifetimeSystem, physics::PhysicsSystem, shot::ShotSystem, spawner::SpawnMobSystem}};

use super::{background::BackgroundStarField, pause::PauseState, score::ScoreHandler};

pub struct PlayingState {
	player: ecs::EntityId,
	pause: bool,
	inputs: [bool; PlayerInput::LAST as usize],
	levels: Vec<Level<LevelPhaseBasicSpawn>>,
	current_level_index: usize,
	background: Option<Arc<RwLock<BackgroundStarField>>>,
	score_handler: Option<Arc<RwLock<ScoreHandler>>>
}

impl PlayingState  {
	pub fn new() -> Self {
		PlayingState {
			player: 0,
			pause: false,
			inputs: [false; PlayerInput::LAST as usize],
			levels: Vec::new(),
			current_level_index: 0,
			background: None,
			score_handler: None,
		}
	}
}

impl states::StateSystems for PlayingState {
	type Systems = tuple_list!(GraphicsSystem, InputSystem, PhysicsSystem, ShotSystem, LifetimeSystem, SpawnMobSystem, FollowMeSystem, AISystem, HealthSystem, AnimationSystem);
}

impl states::State for PlayingState  {
	fn on_enter<'sdl_all, 'l>(&mut self, _systems: &mut SystemHolder, game_services: & mut GameServices<'sdl_all, 'l>, create: bool) {
		println!("ENTER PLAYING ! {}", create);
		if create {
			/*
			systems.add_system::<GraphicsSystem, ()>(game_services.get_world_mut(), ());
			systems.add_system::<InputSystem, ()>(game_services.get_world_mut(), ());
			systems.add_system::<PhysicsSystem, ()>(game_services.get_world_mut(), ());
			systems.add_system::<ShotSystem, ()>(game_services.get_world_mut(), ());
			systems.add_system::<LifetimeSystem, ()>(game_services.get_world_mut(), ());
			systems.add_system::<SpawnMobSystem, ()>(game_services.get_world_mut(), ());
			systems.add_system::<FollowMeSystem, ()>(game_services.get_world_mut(), ());
			systems.add_system::<AISystem, ()>(game_services.get_world_mut(), ());
			systems.add_system::<HealthSystem, ()>(game_services.get_world_mut(), ());
			systems.add_system::<AnimationSystem, ()>(game_services.get_world_mut(), ());
			*/

			self.background = Some(Arc::new(RwLock::new(BackgroundStarField::new(game_services))));
			self.score_handler = Some(Arc::new(RwLock::new(ScoreHandler::new(game_services.resource_manager))));
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

			let level1 = Level::new(vec![Box::new(Level1Start::new()), Box::new(Level1Mid::new()), Box::new(Level1Mid2::new()), Box::new(Level1End::new())], self.background.as_ref().unwrap().clone());
			self.levels.push(level1);
			//self.levels.push(Level::new(vec![Box::new(Level1End::new())]));

			game_services.event_dispatcher.register(self.score_handler.as_ref().unwrap().clone());
		}
	}

	fn update<'sdl_all, 'l>(&mut self, next_state: &mut Option<StateWithSystems>, game_services: &mut GameServices<'sdl_all,'l>) -> bool {
		if self.pause {
			let pause_state: Option<StateWithSystems> = Some(StateWithSystems::new(Box::new(PauseState::new())));
			*next_state = pause_state;
			self.pause = false;
		}
		let player_alive = game_services.get_world().is_alive(&self.player);
		if player_alive {
			if let Some(input) = game_services.get_world_mut().get_component_mut::<InputComponent>(&self.player) {
				input.inputs = self.inputs.clone();
			}
		}

		self.background.as_mut().unwrap().write().unwrap().update(game_services);
		self.score_handler.as_mut().unwrap().write().unwrap().update(&mut game_services.resource_manager, &mut game_services.renderer);

		if self.current_level_index < self.levels.len() {
			if ! self.levels[self.current_level_index].update(game_services) {
				self.current_level_index += 1;
			}
			player_alive
		} else {
			// Victory if player_alive
			if player_alive {
				println!("No more levels.");
			}
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
