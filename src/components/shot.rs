
pub enum ShotType {
	PLAYER,
	ENEMY
}

pub struct ShotComponent {
	pub shot_type: ShotType
}

impl ShotComponent {
	pub fn new(shot_type: ShotType) -> Self {
		ShotComponent {
			shot_type: shot_type
		}
	}
}

impl Default for ShotComponent {
	fn default() -> Self {
		ShotComponent::new(ShotType::PLAYER)
	}
}
