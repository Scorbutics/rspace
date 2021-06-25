use std::thread::sleep;
use std::time::Duration;

use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use crate::core::common;
use crate::core::common::GameServices;
use crate::core::ecs::Runnable;
use crate::core::ecs::SystemHolder;
use crate::core::ecs::WeakRunnable;
use crate::core::ecs::World;
use crate::core::renderers::SdlDrawContext;
use crate::core::renderers::SdlRenderer;
use crate::core::renderers::SdlResourceManager;
use crate::core::states::StateDispatcher;
use crate::core::states::StateSystems;
use crate::sdl2;
use crate::systems::ai::AISystem;
use crate::systems::animation::AnimationSystem;
use crate::systems::graphics::GraphicsSystem;
use crate::systems::health::HealthSystem;
use crate::systems::input::InputSystem;
use crate::systems::lifetime::LifetimeSystem;
use crate::systems::physics::PhysicsSystem;
use crate::systems::shot::ShotSystem;
use crate::systems::spawner::SpawnMobSystem;

pub struct Game<'sdl_all, 'game> {
	pub world: World,
	systems: SystemHolder,
	state: StateDispatcher,
	renderer: Option<SdlRenderer>,
	resource_manager: Option<SdlResourceManager<'sdl_all>>,
	game_services: Option<GameServices<'sdl_all, 'game>>,
	global_runnables: Vec<WeakRunnable>,
	last_ms: u64
}

pub trait RunnableNewable : Runnable {
	fn new<'sdl_all, 'game>(game_services: &mut GameServices<'sdl_all, 'game>) -> Self;
}

impl<'sdl_all, 'game> Game<'sdl_all, 'game> {

	pub fn new<T: StateSystems + 'static>(first_state: Box<T>) -> Self {
		let mut game = Game {
			world: World::new(),
			systems: SystemHolder::new(),
			state: StateDispatcher::new(),
			renderer: Option::None,
			resource_manager: Option::None,
			game_services: Option::None,
			global_runnables: Vec::new(),
			last_ms: 0
		};
		game.state.enqueue_state(first_state);
		game.systems.add_system::<GraphicsSystem, ()>(&mut game.world, ());
		game.systems.add_system::<InputSystem, ()>(&mut game.world, ());
		game.systems.add_system::<PhysicsSystem, ()>(&mut game.world, ());
		game.systems.add_system::<ShotSystem, ()>(&mut game.world, ());
		game.systems.add_system::<LifetimeSystem, ()>(&mut game.world, ());
		game.systems.add_system::<SpawnMobSystem, ()>(&mut game.world, ());
		game.systems.add_system::<AISystem, ()>(&mut game.world, ());
		game.systems.add_system::<HealthSystem, ()>(&mut game.world, ());
		game.systems.add_system::<AnimationSystem, ()>(&mut game.world, ());
		game
	}

	fn update_global_runnables<'l>(global_runnables: &mut Vec<WeakRunnable>, game_services: &mut GameServices<'sdl_all, 'l>) {
		let mut i = 0;
		while i < global_runnables.len() {
			let w_runnable = &mut global_runnables[i];
			if let Some(runnable) = w_runnable.upgrade() {
				runnable.write().unwrap().run(game_services);
				i += 1;
			} else {
				global_runnables.remove(i);
			}
		}
	}

	pub fn run(&'game mut self, canvas: Canvas<Window>, draw_context: &'sdl_all SdlDrawContext, texture_creator: &'sdl_all TextureCreator<WindowContext>) -> Result<(), String> {
		self.renderer = Some(SdlRenderer::new(canvas));
		self.renderer.as_mut().unwrap().clear();
		self.renderer.as_mut().unwrap().present();
		self.resource_manager = Some(SdlResourceManager::new(draw_context, texture_creator));

		self.game_services = Some(GameServices::new(&mut self.world, self.resource_manager.as_mut().unwrap(), self.renderer.as_mut().unwrap(), draw_context));
		self.state.update(&mut self.systems, &mut self.global_runnables, self.game_services.as_mut().unwrap());

		let mut event_pump = draw_context.event_pump()?;
		'running: loop {
			// get the inputs here
			for event in event_pump.poll_iter() {
				if self.state.dispatch_event(&event) {
					break 'running;
				}
			}

			let diff_ms = common::current_time_ms() - self.last_ms;
			// Around 60 FPS
			if diff_ms < 15 {
				sleep(Duration::from_millis(diff_ms));
			}

			self.last_ms = common::current_time_ms();
			let game_services = self.game_services.as_mut().unwrap();
			self.systems.update(game_services);
			Self::update_global_runnables(&mut self.global_runnables, game_services);
			if ! self.state.update(&mut self.systems, &mut self.global_runnables, game_services) {
				break 'running;
			}
			game_services.renderer.update(game_services.resource_manager);

		}
		Ok(())
	}

}
