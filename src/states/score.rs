use sdl2::{rect::Rect};

use crate::{components::health::DeathEvent, core::{events::EventObserver, renderers::{Renderable, SdlRenderer, SdlResourceManager}, resources::{FontDetails}}};

pub struct ScoreHandler {
	score: u32,
	score_texture_index: i64,
	score_rect: Rect,
	font_index: i64,
	dirty: bool
}

impl ScoreHandler {
	pub fn new<'current, 'sdl_all>(resource_manager: &'current mut SdlResourceManager<'sdl_all>) -> Self {
		let font = resource_manager.load_font(&FontDetails { path: "I-pixel-u.ttf".to_string(), size: 16 }).unwrap();
		let (texture_index, texture_rect) = resource_manager.text_to_texture(font, "SCORE 0", None).unwrap();
		ScoreHandler { score: 0, score_texture_index: texture_index, score_rect: texture_rect, font_index: font, dirty: false }
	}

	pub fn update<'sdl_all>(&mut self, resource_manager: &mut SdlResourceManager<'sdl_all>, renderer: &mut SdlRenderer) {
		if self.dirty {
			let (_, texture_rect) = resource_manager.text_to_texture(self.font_index, format!("SCORE {}", self.score).as_str(), Some(self.score_texture_index)).unwrap();
			self.score_rect = texture_rect;
			self.dirty = false;
		}
		let renderable = Renderable::new(self.score_texture_index, None, Some(self.score_rect), i64::MAX - 1);
		renderer.push_renderable(renderable);
	}

	pub fn score(&self) -> u32 {
		self.score
	}
}

impl EventObserver<DeathEvent> for ScoreHandler {
	fn on_event_mut(&mut self, _data: &DeathEvent) {
		self.score += 5;
		self.dirty = true;
	}
}
