pub struct LifetimeComponent {
	pub life_timer_end: u64
}

impl LifetimeComponent {
	pub fn new(life_timer_end: u64) -> Self {
		LifetimeComponent {
			life_timer_end: life_timer_end
		}
	}
}

impl Default for LifetimeComponent {
	fn default() -> Self {
		LifetimeComponent::new(0)
	}
}
