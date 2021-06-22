use crate::levels::phases::TrajectoryType;


pub enum SpawnerType {
	CIRCLE,
	POINT,
	LINEAR
}

pub struct SpawnerComponent {
	pub trajectory_type: TrajectoryType,
	pub spawner_type: SpawnerType,
	pub countdown: usize,
	pub shot_frequency_factor: f32,
	pub frequency_ms: u32,
	pub last_spawn_ms: u64,
	pub luck_percents: f32,
	pub num: u16,
	pub propulsion: f32,
	pub max_angle: f32,
	pub randomize_pos: bool,
	pub symetric_alternate_pos: bool
}

impl SpawnerComponent {
	pub fn new(spawner_type: SpawnerType, frequency_ms: u32, luck_percents: f32, num: u16, propulsion: f32, max_angle: f32, trajectory_type: TrajectoryType) -> Self {
		SpawnerComponent {
			spawner_type: spawner_type,
			frequency_ms: frequency_ms,
			last_spawn_ms: 0,
			luck_percents: luck_percents,
			num: num,
			propulsion: propulsion,
			max_angle: max_angle,
			randomize_pos: false,
			symetric_alternate_pos: false,
			countdown: usize::MAX,
			shot_frequency_factor: 1.0,
			trajectory_type: trajectory_type
		}
	}
}

impl Default for SpawnerComponent {
	fn default() -> Self {
		SpawnerComponent::new(SpawnerType::POINT, 0, 0.0, 0, 0.0, std::f32::consts::PI * 2.0, TrajectoryType::BasicLinear)
	}
}
