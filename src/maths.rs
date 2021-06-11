use crate::{components::{hitbox::HitboxComponent, transform::TransformComponent}, core::ecs::{EntityId, World}};

pub fn center(world: &World, entity_id: &EntityId) -> (f32, f32) {
	let target_pos = world.get_component::<TransformComponent>(&entity_id).unwrap();
	let target_hitbox = world.get_component::<HitboxComponent>(&entity_id).unwrap();
	(target_pos.x - target_hitbox.hitbox.width() as f32 / 2.0 + target_hitbox.hitbox.x as f32, target_pos.y - target_hitbox.hitbox.height() as f32 / 2.0 + target_hitbox.hitbox.y as f32)
}

pub fn angle_between_pos(src: (f32, f32), dst: (f32, f32)) -> f32 {
	let angle;
	if dst.0 == src.0 {
		angle = if dst.1 > src.1 { std::f32::consts::PI/2.0 } else { - std::f32::consts::PI/2.0 };
	} else {
		angle = f32::atan2(dst.1 - src.1, dst.0 - src.0);
	}
	angle
}

pub fn distance_squared(current_pos: (f32, f32), target_pos: (f32, f32)) -> f32 {
	(target_pos.0 - current_pos.0) * (target_pos.0 - current_pos.0) + (target_pos.1 - current_pos.1) * (target_pos.1 - current_pos.1)
}
