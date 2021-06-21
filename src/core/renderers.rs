use std::borrow::Borrow;
use std::rc::Rc;

use sdl2::{Sdl, render};
use sdl2::VideoSubsystem;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::ttf::{Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};

use super::resources::{FontDetails, ResourceLoader};
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
	font_manager: FontManager<'sdl_all>
}

impl<'sdl_all> SdlResourceManager<'sdl_all> {
	pub fn new(draw_context: &'sdl_all SdlDrawContext, texture_creator: &'sdl_all TextureCreator<WindowContext>) -> Self {
		SdlResourceManager {
			texture_manager: TextureManager::new(texture_creator),
			font_manager: FontManager::new(&draw_context.font_context)
		}
	}

	pub fn load_shared_texture(&mut self, filename: &str) -> Result<i64, String> {
		match self.texture_manager.load_shared(filename) {
			Ok(t) => Ok(t.1),
			Err(err) => { return Err(err); },
		}
	}

	pub fn load_font(&mut self, font_details: &FontDetails) -> Result<Rc<Font>, String> {
		match self.font_manager.load_shared(font_details) {
			Ok(t) => Ok(t.0),
			Err(err) => { return Err(err); },
		}
	}

	pub fn get_texture(&self, texture_index: i64) -> Option<&Texture<'sdl_all>> {
		self.texture_manager.from_index(texture_index)
	}

	pub fn get_texture_mut(&mut self, texture_index: i64) -> Option<&mut Texture<'sdl_all>> {
		self.texture_manager.from_index_mut(texture_index)
	}

	pub fn load_unique_texture(&mut self, filename: &str) -> Result<i64, String> {
		match self.texture_manager.load_unique(filename) {
			Ok(t) => Ok(t.1),
			Err(err) => { return Err(err); },
		}
	}

}

#[derive(PartialEq, Eq)]
pub struct Renderable {
	pub src: Option<Rect>,
	pub dst: Option<Rect>,
	pub texture_index: i64,
	pub z: i64
}

impl Renderable {
	pub fn new(texture_index: i64, src: Option<Rect>, dst: Option<Rect>, z: i64) -> Self {
		Renderable {
			texture_index: texture_index,
			src: src,
			dst: dst,
			z: z
		}
	}
}

impl PartialOrd for Renderable {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.z.partial_cmp(&other.z)
	}
}

impl Ord for Renderable {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.z.cmp(&other.z)
	}
}

// TODO SceneGraph, or QuadTree, or I don't know...
pub struct SdlRenderer {
	canvas: Canvas<Window>,
	renderables: Vec<Renderable>
}

impl SdlRenderer {
	pub fn new(canvas: Canvas<Window>) -> Self {
		println!("Using SDL_Renderer \"{}\"", canvas.info().name);
		SdlRenderer {
			canvas: canvas,
			renderables: Vec::new()
		}
	}

	pub fn clear(&mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.clear();
	}

	fn render<'sdl_all>(canvas: &mut Canvas<Window>, resource_manager: &SdlResourceManager<'sdl_all>, texture_index: i64, src: Option<Rect>, dst: Option<Rect>) -> Result<(), String> {
		let texture = resource_manager.get_texture(texture_index);
		if let Some(value) = texture {
			canvas.copy(value.borrow(), src, dst)
		} else {
			Err(String::new())
		}
	}

	pub fn present(&mut self) {
		self.canvas.present();
	}

	pub fn update<'sdl_all>(&mut self, resource_manager: &SdlResourceManager<'sdl_all>) {
		self.clear();
		self.renderables.sort();
		for renderable in &self.renderables {
			Self::render(&mut self.canvas, resource_manager, renderable.texture_index, renderable.src, renderable.dst);
		}
		self.renderables.clear();
		self.present();
	}

	pub fn set_renderables(&mut self, renderables: Vec<Renderable>) {
		self.renderables = renderables
	}

	pub fn push_renderable(&mut self, renderable: Renderable) {
		self.renderables.push(renderable)
	}
}
