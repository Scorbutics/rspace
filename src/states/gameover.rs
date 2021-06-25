use sdl2::{event::Event, keyboard::Keycode, rect::Rect};
use tuple_list::tuple_list_type;
use crate::{core::{common::GameServices, ecs::{WeakRunnable}, renderers::Renderable, resources::FontDetails, states::{self, State, StateWithSystems}}, systems::{ai::AISystem, animation::AnimationSystem, graphics::GraphicsSystem, lifetime::LifetimeSystem, physics::PhysicsSystem}};

pub struct GameOverState {
	text: String,
	score: u32,
	restart: bool,
	font_index: i64,
	texture_index: Vec<i64>,
	rect: Vec<Rect>
}

impl GameOverState {
	pub fn new(victory: bool, score: u32) -> Self {
		GameOverState {
			text: if victory { "VICTORY".to_string() } else { "GAME OVER".to_string() },
			score: score,
			restart: false,
			font_index: 0,
			texture_index: Vec::new(),
			rect: Vec::new()
		}
	}
}

impl states::StateSystems for GameOverState {
	type Systems = tuple_list_type!(GraphicsSystem, AnimationSystem, PhysicsSystem, LifetimeSystem, AISystem);
}

impl State for GameOverState {
	fn on_enter<'sdl_all, 'l>(&mut self, _runnables: &mut Vec<WeakRunnable>, game_services: &mut GameServices<'sdl_all,'l>, create: bool, _last_state_id: Option<usize>) {
		println!("GAME OVER");
		if create {
			self.font_index = game_services.resource_manager.load_font(&FontDetails { path: "I-pixel-u.ttf".to_string(), size: 42 }).unwrap();
			let (i, rect) = game_services.resource_manager.text_to_texture(self.font_index, self.text.as_str(), None).unwrap();
			self.texture_index.push(i);
			self.rect.push(Rect::new(game_services.draw_context.screen_width() as i32 / 2 - rect.w / 2, game_services.draw_context.screen_height() as i32 / 2 - (rect.h * 3 / 2), rect.w as u32, rect.h as u32));

			let (i, rect) = game_services.resource_manager.text_to_texture(self.font_index, format!("SCORE {}", self.score).as_str(), None).unwrap();
			self.texture_index.push(i);
			self.rect.push(Rect::new(game_services.draw_context.screen_width() as i32 / 2 - rect.w / 2, game_services.draw_context.screen_height() as i32 / 2 - rect.h / 2, rect.w as u32, rect.h as u32));
		}
	}

	fn on_event(&mut self, event: &Event) -> bool {
		match event {
			Event::Quit { .. }
			| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
			} | Event::KeyDown {
				keycode: Some(Keycode::Return),
				..
		} => self.restart = true,
			_ => {}
		}
		false
	}

	fn update<'sdl_all, 'l>(&mut self, _next_state: &mut Option<StateWithSystems>, game_services: &mut GameServices<'sdl_all,'l>) -> bool {
		for i in 0.. self.texture_index.len() {
			let renderable = Renderable::new(self.texture_index[i], None, Some(self.rect[i]), 99999);
			game_services.renderer.push_renderable(renderable);
		}
		! self.restart
	}

	fn on_leave<'sdl_all, 'l>(&mut self, _game_services: &mut GameServices<'sdl_all,'l>, _destroy: bool) {
		println!("END {}", _destroy);
	}
}
