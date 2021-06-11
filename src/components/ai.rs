use crate::core::common::current_time_ms;

pub type TimeFunction = dyn Fn(u64, f32, (f32, f32)) -> (f32, f32);

pub struct AIComponent {
	time_movement: Option<Box<TimeFunction>>,
	start_time: u64,
	power: f32
}

impl AIComponent {
	pub fn new(time_movement: Option<Box<TimeFunction>>, power: f32) -> Self {
		AIComponent {
			time_movement: time_movement,
			power: power,
			start_time: current_time_ms()
		}
	}
	pub fn next_position(&self, target: (f32, f32)) -> Option<(f32, f32)> {
		if self.time_movement.is_none() {
			None
		} else {
			let result = Some(self.time_movement.as_ref().unwrap()(current_time_ms() - self.start_time, self.power, target));
			result
		}
	}
}

impl Default for AIComponent {
	fn default() -> Self {
		AIComponent::new(None, 0.0)
	}
}
