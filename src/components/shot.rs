
pub enum ShotType {
	PLAYER,
	ENEMY
}

pub struct ShotComponent {
	pub shot_type: ShotType,
	pub damages: i32
}

impl ShotComponent {
	pub fn new(shot_type: ShotType, damages: i32) -> Self {
		ShotComponent {
			shot_type: shot_type,
			damages: damages
		}
	}
}

impl Default for ShotComponent {
	fn default() -> Self {
		ShotComponent::new(ShotType::PLAYER, 0)
	}
}
