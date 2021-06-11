use std::rc::Rc;

use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::ttf::{Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};

use super::resources::FontDetails;
use super::resources::FontManager;
use super::resources::TextureManager;

pub struct SdlDrawContext {
	pub font_context: Sdl2TtfContext,
	sdl_context: Sdl,
	video_subsystem: VideoSubsystem
}

pub const SQUARE_SIZE: u32 = 16;
pub const PLAYGROUND_WIDTH: u32 = 49;
pub const PLAYGROUND_HEIGHT: u32 = 40;

impl SdlDrawContext {
	pub fn new() -> Self {
		let sdl_context = sdl2::init().unwrap();
		let video_subsystem = sdl_context.video().unwrap();

		SdlDrawContext {
			font_context: sdl2::ttf::init().map_err(|e| e.to_string()).unwrap(),
			sdl_context: sdl_context,
			video_subsystem: video_subsystem
		}
	}

	pub fn spawn_window(&self) -> Window {
		// the window is the representation of a window in your operating system,
		// however you can only manipulate properties of that window, like its size, whether it's
		// fullscreen, ... but you cannot change its content without using a Canvas or using the
		// `surface()` method.
		self.video_subsystem
				.window(
						"SPACE",
						SQUARE_SIZE * PLAYGROUND_WIDTH,
						SQUARE_SIZE * PLAYGROUND_HEIGHT,
				)
				.position_centered()
				.build()
				.map_err(|e| e.to_string()).unwrap()
	}

	pub fn event_pump(&self) -> Result<sdl2::EventPump, String> {
		self.sdl_context.event_pump()
	}

	pub fn screen_width(&self) -> u32 {
		SQUARE_SIZE * PLAYGROUND_WIDTH
	}

	pub fn screen_height(&self) -> u32 {
		SQUARE_SIZE * PLAYGROUND_HEIGHT
	}
}

pub struct SdlResourceManager<'sdl_all> {
	texture_manager: TextureManager<'sdl_all, WindowContext>,
	font_manager: FontManager<'sdl_all>,
	textures: Vec<Rc<Texture<'sdl_all>>>
}

impl<'sdl_all> SdlResourceManager<'sdl_all> {
	pub fn new(draw_context: &'sdl_all SdlDrawContext, texture_creator: &'sdl_all TextureCreator<WindowContext>) -> Self {
		SdlResourceManager {
			texture_manager: TextureManager::new(texture_creator),
			font_manager: FontManager::new(&draw_context.font_context),
			textures: Vec::new()
		}
	}

	pub fn load_texture(&mut self, filename: &str) -> Result<usize, String> {
		let texture = self.texture_manager.load(filename);
		match texture {
			Ok(t) => self.textures.push(t),
			Err(err) => { return Err(err); },
		}
		Ok(self.textures.len() - 1)
	}

	pub fn load_font(&mut self, font_details: &FontDetails) -> Result<Rc<Font>, String> {
		self.font_manager.load(font_details)
	}

	pub fn get_texture(&self, texture_index: usize) -> &Texture<'sdl_all> {
		&self.textures[texture_index]
	}
}

pub struct SdlRenderer {
	canvas: Canvas<Window>
}

impl SdlRenderer {
	pub fn new(canvas: Canvas<Window>) -> Self {
		println!("Using SDL_Renderer \"{}\"", canvas.info().name);
		SdlRenderer {
			canvas: canvas
		}
	}

	pub fn clear(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
	}

	pub fn render<'sdl_all>(&mut self, resource_manager: &SdlResourceManager<'sdl_all>, texture_index: usize, src: Option<Rect>, dst: Option<Rect>) -> Result<(), String> {
		let texture = resource_manager.get_texture(texture_index);
		self.canvas.copy(texture, src, dst)
	}

	pub fn present(&mut self) {
		self.canvas.present();
	}
}

