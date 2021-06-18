use rand::Rng;

use crate::core::common::{self, current_time_ms};

use super::input::State;

pub type DestinationPoint = (f32, f32);

pub struct TrajectorySequence {
	points: Vec<DestinationPoint>,
	start_time_ms: u64,
	shoot_start_time_ms: u64,
	shoot_start_interval_time_ms: u64,
	shoot_index: usize,
	pub shoot_num: usize,
	shoot_delay_ms: u64,
	pub shoot_interval_ms: u64
}

impl TrajectorySequence {
	pub fn new() -> Self {
		Self::wait(0)
	}
	pub fn wait(start_time_ms: u64) -> Self {
		TrajectorySequence {
			points: Vec::new(),
			start_time_ms: start_time_ms,
			shoot_start_time_ms: 0,
			shoot_start_interval_time_ms: 0,
			shoot_delay_ms: 0,
			shoot_num: 0,
			shoot_index: 0,
			shoot_interval_ms: 300
		}
	}
	pub fn push(&mut self, point: DestinationPoint) {
		self.points.push(point);
	}

	pub fn last(&self) -> Option<&DestinationPoint> {
		self.points.last()
	}

	pub fn can_shoot(&mut self) -> bool {
		if self.shoot_delay_ms == 0 {
			return false;
		}
		let current_time = common::current_time_ms();
		let can_start_shoot = (current_time as i64 - self.shoot_start_time_ms as i64) >= self.shoot_delay_ms as i64;
		if can_start_shoot {
			self.shoot_index = 0;
			self.shoot_start_time_ms = current_time;
			self.shoot_start_interval_time_ms = self.shoot_start_time_ms.clone();
			false
		} else if self.shoot_index < self.shoot_num {
			let can_shoot = self.shoot_interval_ms != 0 && (current_time as i64 - self.shoot_start_interval_time_ms as i64) >= self.shoot_interval_ms as i64;
			if can_shoot {
				self.shoot_start_interval_time_ms = current_time;
				self.shoot_index += 1;
				true
			} else {
				false
			}
		} else {
			false
		}
	}

	pub fn set_shoot_delay(&mut self, shoot_delay_ms: u64) {
		self.shoot_delay_ms = shoot_delay_ms;
		let mut rng = rand::thread_rng();
		self.shoot_start_time_ms = common::current_time_ms() + rng.gen_range(0, self.shoot_delay_ms.clone());
		self.shoot_start_interval_time_ms = self.shoot_start_time_ms.clone();
	}
}

pub struct AIComponent {
	trajectories: Vec<TrajectorySequence>,
	current_trajectory: usize,
	current_point: usize,
	pub state: State,
	pub last_state: State,
	pub shot_power: f32,
	pub speed: f32
}

impl AIComponent {
	pub fn new() -> Self {
		AIComponent {
			trajectories: Vec::new(),
			current_point: 0,
			current_trajectory: 0,
			state: State::Stand,
			last_state: State::Stand,
			shot_power: 5.0,
			speed : 5.0
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

	pub fn can_shoot(&mut self) -> bool {
		if self.current_trajectory >= self.trajectories.len() {
			false
		} else {
			self.trajectories[self.current_trajectory].can_shoot()
		}
	}
}

impl Default for AIComponent {
	fn default() -> Self {
		AIComponent::new()
	}
}
