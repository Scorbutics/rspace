use sdl2::rect::Rect;

pub struct SpriteComponent {
	pub sprite: usize,
	pub graphic_box: Rect
}

impl Default for SpriteComponent {
	fn default() -> Self {
		SpriteComponent {
			sprite: 0,
			graphic_box: Rect::new(0,0, 1, 1)
		}
	}
}

impl SpriteComponent {
	pub fn new(texture_index: usize, width: u32, height: u32) -> Self {
		SpriteComponent {
			sprite: texture_index,
			graphic_box: Rect::new(0,0, width, height)
		}
	}
}