pub struct TransformComponent {
	pub x: f32,
	pub y: f32
}

impl TransformComponent {
	pub fn new(x: f32, y: f32) -> Self {
		TransformComponent{
			x : x,
			y : y
		}
	}
}

impl Default for TransformComponent {
	fn default() -> Self {
		TransformComponent::new(0.0, 0.0)
	}
}

impl Clone for TransformComponent {
	fn clone(&self) -> Self {
		TransformComponent {
			x: self.x.clone(),
			y: self.y.clone()
		}
	}
}

impl std::fmt::Display for TransformComponent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}; {})", self.x, self.y)
	}
}
