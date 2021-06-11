use std::sync::{Arc, RwLock};

use tuple_list::tuple_list_type;

use crate::{components::{hitbox::HitboxComponent, shot::{ShotComponent, ShotType}, transform::TransformComponent}, core::{common::{GameServices}, ecs::{Runnable, System, SystemComponents, SystemNewable}}};

pub struct ShotSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for ShotSystem {
	type Components = tuple_list_type!(ShotComponent, HitboxComponent, TransformComponent);
}

impl SystemNewable<ShotSystem, ()> for ShotSystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		ShotSystem {
			base: base
		}
	}
}

impl Runnable for ShotSystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for entity in self.base.read().unwrap().iter_entities() {
			let world = game_services.get_world_mut();
			let shot = world.get_component::<ShotComponent>(entity).unwrap();

			// TODO kill enemies / player if collision
			// (For this game we can code a naive collision system that will iterate all entities except self and check collisions with it)
			match shot.shot_type {
				ShotType::PLAYER => {},
				ShotType::ENEMY => {},
			}
		}
	}
}
