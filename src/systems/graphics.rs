use std::sync::{Arc, RwLock};

use sdl2::rect::Rect;
use tuple_list::tuple_list_type;

use crate::{components::{sprite::{SpriteComponent, SpritesheetOrientation}, transform::TransformComponent}, core::{common::{self, GameServices}, ecs::{Runnable, System, SystemComponents, SystemNewable}}};

pub struct GraphicsSystem {
	base: Arc<RwLock<System>>
}

impl SystemComponents for GraphicsSystem {
	type Components = tuple_list_type!(TransformComponent, SpriteComponent);
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
			let sprite = game_services.get_world().get_component::<SpriteComponent>(entity_id).unwrap();
			let pos = game_services.get_world().get_component::<TransformComponent>(entity_id).unwrap();
			let rect = Rect::new(pos.x as i32 + sprite.graphic_box.x, pos.y as i32 + sprite.graphic_box.y, sprite.graphic_box.width(), sprite.graphic_box.height());
			let sprite_index = sprite.sprite;
			let src;
			if sprite.spritesheet.is_some() {
				let sprite = game_services.get_world_mut().get_component_mut::<SpriteComponent>(entity_id).unwrap();
				let spritesheet = sprite.spritesheet.as_mut().unwrap();
				let graphic_box = Rect::new(sprite.spritesheet_index.0 as i32 * spritesheet.width as i32, sprite.spritesheet_index.1 as i32 * spritesheet.height as i32, spritesheet.width, spritesheet.height);
				let (sprite_index, frame_num) = match spritesheet.orientation {
					SpritesheetOrientation::HORIZONTAL => {
						(sprite.spritesheet_index.0 as i16, spritesheet.num_width)
					},
					SpritesheetOrientation::VERTICAL =>  {
						(sprite.spritesheet_index.1 as i16, spritesheet.num_height)
					},
				};

				let delay_off = (! sprite.animation_pause) && common::current_time_ms() - sprite.animation_start_time >= sprite.animation_delay;
				if delay_off {
					let next_index = if sprite_index + 1 >= frame_num as i16 && sprite.animation_direction == 1 || sprite_index == 0 && sprite.animation_direction == -1 {
						if sprite.animation_loop && sprite.animation_direction == 1 {
							0
						} else if sprite.animation_loop && sprite.animation_direction == -1 {
							frame_num as i16 - 1
						} else {
							sprite_index as i16
						}
					} else {
						sprite_index + sprite.animation_direction
					};
					sprite.spritesheet_index = if spritesheet.orientation == SpritesheetOrientation::HORIZONTAL { (next_index as usize, sprite.spritesheet_index.1) } else { (sprite.spritesheet_index.0, next_index as usize)};
					sprite.animation_start_time = common::current_time_ms();
				}

				src = Some(graphic_box);
			} else {
				src = Option::None;
			}
			game_services.renderer.render(game_services.resource_manager, sprite_index, src,
					Some(rect)).expect("Error while rendering");
		}
	}
}
