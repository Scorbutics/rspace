use sdl2::rect::Rect;

pub enum SpritesheetOrientation {
	HORIZONTAL,
	VERTICAL
}

pub struct Spritesheet {
	pub num: usize,
	pub orientation: SpritesheetOrientation,
	pub width: u32,
	pub height: u32,
}

impl Spritesheet {
	pub fn new(num: usize, orientation: SpritesheetOrientation, width: u32, height: u32) -> Self {
		Spritesheet {
			num: num,
			orientation: orientation,
			width: width,
			height: height
		}
	}
}

pub struct SpriteComponent {
	pub sprite: usize,
	pub graphic_box: Rect,
	pub spritesheet: Option<Spritesheet>,
	pub spritesheet_index: usize,
	pub animation_delay: u64,
	pub animation_start_time: u64,
}

impl Default for SpriteComponent {
	fn default() -> Self {
		SpriteComponent {
			sprite: 0,
			graphic_box: Rect::new(0,0, 1, 1),
			spritesheet_index: 0,
			spritesheet: None,
			animation_start_time : 0,
			animation_delay : 0,
		}
	}
}

impl SpriteComponent {
	pub fn new(texture_index: usize, width: u32, height: u32) -> Self {
		SpriteComponent {
			sprite: texture_index,
			graphic_box: Rect::new(0,0, width, height),
			spritesheet: None,
			spritesheet_index: 0,
			animation_start_time : 0,
			animation_delay : 0,
		}
	}
}
