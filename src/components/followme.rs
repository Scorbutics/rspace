use crate::core::ecs::EntityId;

pub struct FollowMeComponent {
	pub leader: EntityId,
	pub speed: f32
}

impl FollowMeComponent {
	pub fn new(leader: EntityId, speed: f32) -> Self {
		FollowMeComponent {
			leader: leader,
			speed: speed
		}
	}
}

impl Default for FollowMeComponent {
	fn default() -> Self {
		FollowMeComponent::new(0, 0.0)
	}
}
