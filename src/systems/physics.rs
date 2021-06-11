use std::sync::{Arc, RwLock};

use tuple_list::tuple_list_type;

use crate::{components::{force::ForceComponent, transform::TransformComponent}, core::{common::GameServices, ecs::{Runnable, System, SystemComponents, SystemNewable}}};


pub struct PhysicsSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for PhysicsSystem {
	type Components = tuple_list_type!(TransformComponent, ForceComponent);
}

impl SystemNewable<PhysicsSystem, ()> for PhysicsSystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		PhysicsSystem {
			base: base
		}
	}
}

impl Runnable for PhysicsSystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for entity in self.base.read().unwrap().iter_entities() {
			let world = game_services.get_world_mut();
			let force = world.get_component_mut::<ForceComponent>(entity).unwrap();
			force.vx += force.ax;
			force.vy += force.ay;
			force.ax = 0.0; force.ay = 0.0;

			let (vx, vy) = (force.vx, force.vy);

			// On pourrait également ici avoir des force(s) de frottements, proportionnelles à la vélocité
			// Avec un coefficient "A"
			// f.x = -A * force.vx;
			// f.y = -A * force.vy;
			//force.vx += f.x; force.vy = f.y;

			let pos = world.get_component_mut::<TransformComponent>(entity).unwrap();
			pos.x += vx;
			pos.y += vy;
		}
	}
}
