use crate::core::common::current_time_ms;

pub type DestinationPoint = (f32, f32);

pub struct TrajectorySequence {
	points: Vec<DestinationPoint>,
	start_time_ms: u64,
}

impl TrajectorySequence {
	pub fn new() -> Self {
		Self::wait(0)
	}
	pub fn wait(start_time_ms: u64) -> Self {
		TrajectorySequence {
			points: Vec::new(),
			start_time_ms: start_time_ms
		}
	}
	pub fn push(&mut self, point: DestinationPoint) {
		self.points.push(point);
	}

	pub fn last(&self) -> Option<&DestinationPoint> {
		self.points.last()
	}
}

pub struct AIComponent {
	trajectories: Vec<TrajectorySequence>,
	current_trajectory: usize,
	current_point: usize
}

impl AIComponent {
	pub fn new() -> Self {
		AIComponent {
			trajectories: Vec::new(),
			current_point: 0,
			current_trajectory: 0,
		}
	}
	pub fn next_position(&mut self, actual_pos: &DestinationPoint, tolerance: &f32) -> Option<(f32, f32)> {
		if self.current_trajectory >= self.trajectories.len() {
			None
		} else {
			let pattern = &self.trajectories[self.current_trajectory];
			if pattern.start_time_ms <= current_time_ms() {
				if self.current_point >= pattern.points.len() {
					self.current_trajectory += 1;
					self.current_point = 0;
					self.next_position(actual_pos, tolerance)
				} else {
					let point = self.current_point;
					let target_pos = pattern.points[point];
					if (actual_pos.0 - target_pos.0).abs() <= (*tolerance + 1.0) && (actual_pos.1 - target_pos.1).abs() <= (*tolerance + 1.0) {
						self.current_point += 1;
					}
					Some(target_pos)
				}
			} else {
				Some(*actual_pos)
			}
		}
	}

	pub fn add_movement_pattern(&mut self, pattern: TrajectorySequence) {
		self.trajectories.push(pattern);
	}

	pub fn set_movement_patterns(&mut self, patterns: Vec<TrajectorySequence>) {
		self.trajectories = patterns;
	}
}

impl Default for AIComponent {
	fn default() -> Self {
		AIComponent::new()
	}
}
