use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;

use crate::core::common::GameServices;
use crate::core::ecs::SystemHolder;
use crate::core::ecs::World;
use crate::core::renderers::SdlDrawContext;
use crate::core::renderers::SdlRenderer;
use crate::core::renderers::SdlResourceManager;
use crate::core::states::State;
use crate::core::states::StateDispatcher;
use crate::sdl2;
use crate::systems::ai::AISystem;
use crate::systems::followme::FollowMeSystem;
use crate::systems::graphics::GraphicsSystem;
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
	game_services: Option<GameServices<'sdl_all, 'game>>
}

impl<'sdl_all, 'game> Game<'sdl_all, 'game> {

	pub fn new(first_state: Box<dyn State>) -> Self {
		let mut game = Game {
			world: World::new(),
			systems: SystemHolder::new(),
			state: StateDispatcher::new(),
			renderer: Option::None,
			resource_manager: Option::None,
			game_services: Option::None
		};
		game.state.enqueue_state(first_state);
		game.systems.add_system::<GraphicsSystem, ()>(&mut game.world, ());
		game.systems.add_system::<InputSystem, ()>(&mut game.world, ());
		game.systems.add_system::<PhysicsSystem, ()>(&mut game.world, ());
		game.systems.add_system::<ShotSystem, ()>(&mut game.world, ());
		game.systems.add_system::<LifetimeSystem, ()>(&mut game.world, ());
		game.systems.add_system::<SpawnMobSystem, ()>(&mut game.world, ());
		game.systems.add_system::<FollowMeSystem, ()>(&mut game.world, ());
		game.systems.add_system::<AISystem, ()>(&mut game.world, ());
		game
	}

	pub fn run(&'game mut self, canvas: Canvas<Window>, draw_context: &'sdl_all SdlDrawContext, texture_creator: &'sdl_all TextureCreator<WindowContext>) -> Result<(), String> {
		self.renderer = Some(SdlRenderer::new(canvas));
		self.renderer.as_mut().unwrap().clear();
		self.renderer.as_mut().unwrap().present();
		self.resource_manager = Some(SdlResourceManager::new(draw_context, texture_creator));

		self.game_services = Some(GameServices::new(&mut self.world, self.resource_manager.as_mut().unwrap(), self.renderer.as_mut().unwrap(), draw_context));
		self.state.update(self.game_services.as_mut().unwrap());

		let mut event_pump = draw_context.event_pump()?;
		let mut frame: u32 = 0;
		'running: loop {
			// get the inputs here
			for event in event_pump.poll_iter() {
				if self.state.dispatch_event(&event) {
					break 'running;
				}
			}

			// update the game loop here
			if frame >= 30 {
				self.systems.update(self.game_services.as_mut().unwrap());
				self.state.update(self.game_services.as_mut().unwrap());
				self.game_services.as_mut().unwrap().renderer.present();
				self.game_services.as_mut().unwrap().renderer.clear();
				frame = 0;
			}

			frame += 1;
		}
		Ok(())
	}

}
