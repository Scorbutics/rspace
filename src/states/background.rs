use fastapprox::{fast};
use sdl2::{pixels::Color, rect::Rect, render::BlendMode};

use crate::{core::{common::{self, GameServices}, events::EventObserver, renderers::Renderable}, levels::{phase_basic_spawn::LevelPhaseBasicSpawn}};

pub struct BackgroundStarField {
	src_width: u32,
	src_height: u32,
	texture_index_layers: Vec<i64>,
	scroll_start_ms: Vec<u64>,
	scroll_speed_ms: Vec<f64>,
	next_scroll_speed_ms: f64,
	color_fade_start_ms: u64,
	color_start: Color,
	color_end: Color,
}

const BACKGROUND_COLOR_FADE_SPEED_MS: u64 = 30000;
const SCROLL_OPACITY_LEVEL: usize = 30;
const LAYER_INDEX_MAX: usize = 3;
const BACKGROUND_Z_INDEX: i64 = -10;

impl BackgroundStarField {
	pub fn new<'sdl_all, 'l>(game_services: &mut GameServices<'sdl_all, 'l>) -> Self {
		let mut resources = Vec::with_capacity(LAYER_INDEX_MAX);

		let next_scroll_speed_ms = 30.0;
		let mut scroll_speed_ms = Vec::new();
		scroll_speed_ms.push(next_scroll_speed_ms as f64);

		let fog_texture = game_services.resource_manager.load_unique_texture("fog.png").unwrap();
		let texture = game_services.resource_manager.get_texture_mut(fog_texture).unwrap();
		texture.set_blend_mode(BlendMode::Blend);
		texture.set_alpha_mod(20);
		resources.push(fog_texture);

		for layer_index in 0..LAYER_INDEX_MAX {
			let name = format!("star_background-{}.png", (layer_index + 1));
			let layer_texture = game_services.resource_manager.load_unique_texture(name.as_str()).unwrap();
			let texture = game_services.resource_manager.get_texture_mut(layer_texture).unwrap();
			texture.set_blend_mode(BlendMode::Blend);
			texture.set_alpha_mod(((layer_index + 1) * SCROLL_OPACITY_LEVEL) as u8);
			scroll_speed_ms.push(next_scroll_speed_ms / (layer_index + 1) as f64);
			resources.push(layer_texture);
		}

		BackgroundStarField {
			src_width: 320,
			src_height: 320,
			scroll_speed_ms: scroll_speed_ms,
			next_scroll_speed_ms: next_scroll_speed_ms,
			texture_index_layers: resources,
			scroll_start_ms: vec![common::current_time_ms(); LAYER_INDEX_MAX + 1],
			color_fade_start_ms: common::current_time_ms(),
			color_start: Color::RGB(0, 100, 102),
			color_end: Color::RGB(77, 25, 77)
		}
	}

	fn color_mix_rgb(color_start: &Color, color_end: &Color, alpha: u8, percents: f32) -> Color {
		Color::RGBA(
			(color_start.r as f32 * (1.0 - percents) + color_end.r as f32 * percents) as u8,
			(color_start.g as f32 * (1.0 - percents) + color_end.g as f32 * percents) as u8,
			(color_start.b as f32 * (1.0 - percents) + color_end.b as f32 * percents) as u8,
			alpha
		)
	}

	fn show_layer<'sdl_all, 'l>(&self, texture_index: &i64, layer_index: usize, z: i64, game_services: &mut GameServices<'sdl_all, 'l>) -> bool {
		let mut layer_to_refresh = false;
		let num_background_width = game_services.draw_context.screen_width() / self.src_width + 1;
		let num_background_height = (game_services.draw_context.screen_height() / self.src_height + 1) as i32;
		for tile_width_index in 0..num_background_width {
			for tile_height_index in -num_background_height..num_background_height {
				let mut background_tile = Rect::new((tile_width_index * self.src_width) as i32, 0, self.src_width, self.src_height);
				let scroll_offset_y = (((common::current_time_ms() - self.scroll_start_ms[layer_index]) as u64) as f64 / self.scroll_speed_ms[layer_index]) as i32;
				if scroll_offset_y > game_services.draw_context.screen_height() as i32 {
					layer_to_refresh = true;
				}
				let base_y = (tile_height_index as i32 * self.src_height as i32) as i32;
				let layer_offset_y = ((layer_index * ((tile_height_index + num_background_height) as usize)) / LAYER_INDEX_MAX) as i32;
				background_tile.y = base_y + scroll_offset_y + layer_offset_y;
				let mut renderable = Renderable::new(*texture_index, None, Some(background_tile), z);
				if layer_index % 2 == 0 {
					renderable.flip_horizontal = true;
				}
				game_services.renderer.push_renderable(renderable);
			}
		}
		layer_to_refresh
	}

	pub fn update<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for (index, texture_index) in self.texture_index_layers.iter().enumerate() {
			let z = if index == 0 {
				9999
			} else {
				BACKGROUND_Z_INDEX + index as i64
			};
			if self.show_layer(texture_index, index, z, game_services) {
				self.scroll_start_ms[index] = common::current_time_ms();
				self.scroll_speed_ms[index] = self.next_scroll_speed_ms / (index + 1) as f64;
			}
		}
		let total_elapsed_ms_from_start = (common::current_time_ms() - self.color_fade_start_ms) as f64;
		let frequency = total_elapsed_ms_from_start / BACKGROUND_COLOR_FADE_SPEED_MS as f64;
		let fade_percents = fast::cos((frequency  % (2.0 * std::f64::consts::PI)) as f32);
		game_services.renderer.set_draw_color(Self::color_mix_rgb(&self.color_start, &self.color_end, 150, fade_percents));
	}

}

impl<'playing_state>  EventObserver<LevelPhaseBasicSpawn> for BackgroundStarField {
	fn on_event_mut(&mut self, data: &LevelPhaseBasicSpawn) {
		self.next_scroll_speed_ms = (30.0 / data.hyperspace_speed) * 30.0;
	}
}
