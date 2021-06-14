use std::sync::{Arc, RwLock};

use sdl2::rect::Rect;
use tuple_list::tuple_list_type;

use crate::{components::{followme::FollowMeComponent, force::ForceComponent, hitbox::HitboxComponent, transform::TransformComponent}, core::{common::GameServices, ecs::{EntityId, Runnable, System, SystemComponents, SystemNewable, World}}, game, maths};

pub struct FollowMeSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for FollowMeSystem {
	type Components = tuple_list_type!(TransformComponent, ForceComponent, HitboxComponent, FollowMeComponent);
}

impl SystemNewable<FollowMeSystem, ()> for FollowMeSystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		FollowMeSystem {
			base: base
		}
	}
}

impl FollowMeSystem {
	pub fn check_leader_validity(leader: &EntityId, world: &World) -> bool {
		world.has_component::<TransformComponent>(leader) &&
		world.has_component::<HitboxComponent>(leader)
	}
}

impl Runnable for FollowMeSystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for entity_id in self.base.read().unwrap().iter_entities() {
			let followme = game_services.get_world().get_component::<FollowMeComponent>(entity_id).unwrap();
			let followme_leader = followme.leader;
			let followme_speed = followme.speed;
			if Self::check_leader_validity(&followme_leader, game_services.get_world()) {
				let current_pos = maths::center(game_services.get_world(), entity_id);
				let target_pos = maths::center(game_services.get_world(), &followme_leader);

				let entity_hitbox = game_services.get_world().get_component::<HitboxComponent>(entity_id).unwrap();
				let entity_hitbox_min = std::cmp::min(entity_hitbox.hitbox.width(), entity_hitbox.hitbox.height());
				let followed_hitbox = game_services.get_world().get_component::<HitboxComponent>(&followme_leader).unwrap();
				let followed_hitbox_min = std::cmp::min(followed_hitbox.hitbox.width(), followed_hitbox.hitbox.height());
				let accepted_distance = (entity_hitbox_min + followed_hitbox_min) as f32 / 2.5;

				let force = game_services.get_world_mut().get_component_mut::<ForceComponent>(entity_id).unwrap();
				let distance_squared = maths::distance_squared(current_pos, target_pos);
				if accepted_distance != 0.0 && distance_squared > (accepted_distance * accepted_distance) {
					let angle = maths::angle_between_pos(current_pos, target_pos);
					let factor = followme_speed + 0.6 * (distance_squared / (accepted_distance * accepted_distance));
					force.vx = factor * f32::cos(angle);
					force.vy = factor * f32::sin(angle);
				} else {
					force.vx = 0.0;
					force.vy = 0.0;
				}
			} else {
				game_services.get_world_mut().remove_entity(entity_id);
				println!("LEADER DEAD : {}", *entity_id);
			}
		}
	}
}
