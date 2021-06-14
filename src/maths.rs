use crate::{components::{hitbox::HitboxComponent, transform::TransformComponent}, core::ecs::{EntityId, World}};

pub fn center(world: &World, entity_id: &EntityId) -> (f32, f32) {
	let target_pos = world.get_component::<TransformComponent>(&entity_id).unwrap();
	let target_hitbox = world.get_component::<HitboxComponent>(&entity_id).unwrap();
	(target_pos.x + target_hitbox.hitbox.width() as f32 / 2.0 + target_hitbox.hitbox.x as f32, target_pos.y + target_hitbox.hitbox.height() as f32 / 2.0 + target_hitbox.hitbox.y as f32)
}

pub fn rect_overlap(rect1: &sdl2::rect::Rect, rect2: &sdl2::rect::Rect) -> bool {
	(i32::abs((rect1.x + rect1.w/2) - (rect2.x + rect2.w/2)) * 2 < (rect1.w + rect2.w)) &&
	(i32::abs((rect1.y + rect1.h/2) - (rect2.y + rect2.h/2)) * 2 < (rect1.h + rect2.h))
}

pub fn collision(world: &World, entity1: &EntityId, entity2: &EntityId) -> bool {
	let rect1 = rect(world, entity1);
	let rect2 = rect(world, entity2);
	rect_overlap(&rect1, &rect2)
}

pub fn rect(world: &World, entity: &EntityId) -> sdl2::rect::Rect {
	let entity_pos = world.get_component::<TransformComponent>(entity).unwrap();
	let entity_box = world.get_component::<HitboxComponent>(entity).unwrap();
	sdl2::rect::Rect::new((entity_pos.x + entity_box.hitbox.x as f32) as i32, (entity_pos.y + entity_box.hitbox.y as f32) as i32,
	entity_box.hitbox.width() as u32, entity_box.hitbox.height() as u32)
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

pub fn next_step_to_pos(current_pos: (f32, f32), target_pos: (f32, f32), power: f32) -> (f32, f32) {
	let v = (target_pos.0 - current_pos.0, target_pos.1 - current_pos.1);
	let magnitude = f32::sqrt(distance_squared(current_pos, target_pos));
	if magnitude == 0.0 {
		(0.0, 0.0)
	} else {
		(v.0 * power / magnitude, v.1 * power / magnitude)
	}
}
