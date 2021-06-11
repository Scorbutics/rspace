pub struct SpriteComponent {
	pub sprite: usize
}

impl Default for SpriteComponent {
	fn default() -> Self {
		SpriteComponent {
			sprite: 0
		}
	}
}

impl SpriteComponent {
	pub fn new(texture_index: usize) -> Self {
		SpriteComponent {
			sprite: texture_index
		}
	}
}