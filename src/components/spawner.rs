
pub enum SpawnerType {
	CIRCLE,
	POINT,
	LINEAR
}

pub struct SpawnerComponent {
	pub spawner_type: SpawnerType,
	pub frequency_ms: u32,
	pub last_spawn_ms: u64,
	pub luck_percents: f32,
	pub num: u16,
	pub propulsion: f32,
	pub max_angle: f32
}

impl SpawnerComponent {
	pub fn new(spawner_type: SpawnerType, frequency_ms: u32, luck_percents: f32, num: u16, propulsion: f32, max_angle: f32) -> Self {
		SpawnerComponent {
			spawner_type: spawner_type,
			frequency_ms: frequency_ms,
			last_spawn_ms: 0,
			luck_percents: luck_percents,
			num: num,
			propulsion: propulsion,
			max_angle: max_angle
		}
	}
}

impl Default for SpawnerComponent {
	fn default() -> Self {
		SpawnerComponent::new(SpawnerType::POINT, 0, 0.0, 0, 0.0, std::f32::consts::PI * 2.0)
	}
}
