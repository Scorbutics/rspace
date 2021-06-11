use std::sync::{Arc, RwLock};

use sdl2::rect::Rect;
use tuple_list::tuple_list_type;

use crate::{components::{hitbox::HitboxComponent, sprite::SpriteComponent, transform::TransformComponent}, core::{common::GameServices, ecs::{Runnable, System, SystemComponents, SystemNewable}}};

pub struct GraphicsSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for GraphicsSystem {
	type Components = tuple_list_type!(TransformComponent, SpriteComponent, HitboxComponent);
}

impl SystemNewable<GraphicsSystem, ()> for GraphicsSystem {
	fn new(base: Arc<RwLock<System>>, _none: ()) -> Self {
		GraphicsSystem {
			base: base
		}
	}
}

impl Runnable for GraphicsSystem {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		for entity_id in self.base.read().unwrap().iter_entities() {
			let sprite = game_services.get_world().get_component::<SpriteComponent>(entity_id).unwrap().sprite;
			let pos = game_services.get_world().get_component::<TransformComponent>(entity_id).unwrap();
			let hitbox = game_services.get_world().get_component::<HitboxComponent>(entity_id).unwrap();
			let rect = Rect::new(pos.x as i32 + hitbox.hitbox.x, pos.y as i32 + hitbox.hitbox.y, hitbox.hitbox.width(), hitbox.hitbox.height());
			game_services.renderer.render(game_services.resource_manager, sprite, Option::None,
				Some(rect)).expect("Error while rendering");
		}
	}
}
