use std::time::{SystemTime, UNIX_EPOCH};

use super::{ecs::World, events::EventDispatcher, renderers::{SdlDrawContext, SdlRenderer, SdlResourceManager}};

pub struct GameServices<'sdl_all, 'parent> {
	world: &'parent mut World,
	pub draw_context: &'parent SdlDrawContext,
	pub renderer: &'parent mut SdlRenderer,
	pub resource_manager: &'parent mut SdlResourceManager<'sdl_all>,
	pub event_dispatcher: EventDispatcher
}

impl<'sdl_all, 'parent> GameServices<'sdl_all, 'parent> {
	pub fn new(world: &'parent mut World, resource_manager: &'parent mut SdlResourceManager<'sdl_all>, renderer: &'parent mut SdlRenderer, draw_context: &'parent SdlDrawContext) -> Self {
		GameServices {
			world: world,
			resource_manager: resource_manager,
			renderer: renderer,
			draw_context: draw_context,
			event_dispatcher: EventDispatcher::new()
		}
	}

	pub fn get_world(&self) -> &World { self.world }
	pub fn get_world_mut(&mut self) -> &mut World { self.world }
}

pub fn current_time_ms() -> u64 {
	let start = SystemTime::now();
	let since_the_epoch = start
		.duration_since(UNIX_EPOCH)
		.expect("Time went backwards");
	since_the_epoch.as_secs() * 1000 +
	since_the_epoch.subsec_nanos() as u64 / 1_000_000
}
