pub mod states;
pub mod core;
pub mod systems;
pub mod components;
pub mod factory;
pub mod maths;
pub mod levels;
pub extern crate sdl2;
mod game;

fn main() {
	let draw_context = core::renderers::SdlDrawContext::new();
	//let ttt = self.draw_context.as_mut().unwrap().as_mut();
	let window = draw_context.spawn_window();
	// the canvas allows us to both manipulate the property of the window and to change its content
	// via hardware or software rendering. See CanvasBuilder for more info.
	let canvas;
	match window
		.into_canvas()
		.target_texture()
		.present_vsync()
		.build()
		.map_err(|e| e.to_string()) {
		Ok(c) => {
			canvas = c;
		},
		Err(e) => { panic!("Window cannot be created : {}", e) }
	}

	let texture_creator = canvas.texture_creator();
	let mut game = game::Game::new(Box::new(states::playing::PlayingState::new()));
	match game.run(canvas, &draw_context, &texture_creator) {
		Ok(_) => {},
		Err(e) => panic!("Error during game execution : {}", e),
	}
}
