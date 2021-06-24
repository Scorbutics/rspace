use crate::core::{ecs::EntityId};

pub struct HealthComponent {
	pub health_points: i64
}

impl HealthComponent {
	pub fn new(health_points: i64) -> Self {
		HealthComponent {
			health_points: health_points
		}
	}
}

impl Default for HealthComponent {
	fn default() -> Self {
		HealthComponent::new(0)
	}
}

pub struct DeathEvent {
	pub entity: EntityId
}
