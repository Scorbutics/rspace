use sdl2::rect::Rect;

pub struct HitboxComponent {
	pub hitbox: Rect
}

impl HitboxComponent {
	pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
		HitboxComponent {
			hitbox: Rect::new(x, y, width, height)
		}
	}
}

impl Default for HitboxComponent {
	fn default() -> Self {
		HitboxComponent::new(0, 0, 1, 1)
	}
}
