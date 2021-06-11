pub struct ForceComponent {
	pub ax: f32,
	pub ay: f32,
	pub vx: f32,
	pub vy: f32,
}

impl ForceComponent {
	pub fn new() -> Self {
		ForceComponent {
			ax: 0.0,
			ay: 0.0,
			vx: 0.0,
			vy: 0.0,
		}
	}
}

impl Default for ForceComponent {
	fn default() -> Self {
		ForceComponent::new()
	}
}