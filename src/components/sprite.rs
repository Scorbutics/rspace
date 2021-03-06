use sdl2::rect::Rect;

#[derive(PartialEq, Eq, Hash)]
pub enum SpritesheetOrientation {
	HORIZONTAL,
	VERTICAL
}

pub struct Spritesheet {
	pub num_width: usize,
	pub num_height: usize,
	pub orientation: SpritesheetOrientation,
	pub width: u32,
	pub height: u32,
}

impl Spritesheet {
	pub fn new(num_width: usize, num_height: usize, orientation: SpritesheetOrientation, width: u32, height: u32) -> Self {
		Spritesheet {
			num_width: num_width,
			num_height: num_height,
			orientation: orientation,
			width: width,
			height: height
		}
	}
}

pub struct SpriteComponent {
	pub sprite: i64,
	pub graphic_box: Rect,
	pub spritesheet: Option<Spritesheet>,
	pub spritesheet_index: (usize, usize),
	pub z: i64,
}

impl Default for SpriteComponent {
	fn default() -> Self {
		SpriteComponent {
			sprite: 0,
			z: 0,
			graphic_box: Rect::new(0,0, 1, 1),
			spritesheet_index: (0, 0),
			spritesheet: None,
		}
	}
}

impl SpriteComponent {
	pub fn new(texture_index: i64, width: u32, height: u32, z: i64) -> Self {
		SpriteComponent {
			sprite: texture_index,
			z: z,
			graphic_box: Rect::new(0,0, width, height),
			spritesheet: None,
			spritesheet_index: (0, 0)
		}
	}
}
