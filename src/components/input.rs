use std::convert::TryFrom;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum State {
	MoveRight,
	MoveLeft,
	Stand,
}
pub enum PlayerInput {
	LEFT, RIGHT, UP, DOWN, SHOOT, LAST
}

impl TryFrom<&usize> for PlayerInput {
	type Error = ();

	fn try_from(v: &usize) -> Result<Self, Self::Error> {
		match *v {
			0 => Ok(PlayerInput::LEFT),
			1 => Ok(PlayerInput::RIGHT),
			2 => Ok(PlayerInput::UP),
			3 => Ok(PlayerInput::DOWN),
			4 => Ok(PlayerInput::SHOOT),
			_ => Err(()),
		}
	}
}
pub struct InputComponent {
	pub inputs: [bool; PlayerInput::LAST as usize],
	pub keyboard: bool,
	pub power: f32,
	pub shot_timer_start: u64,
	pub state: State,
	pub last_state: State,
}

impl InputComponent {
	pub fn new(power: f32, keyboard: bool) -> Self {
		InputComponent {
			inputs: [false; PlayerInput::LAST as usize],
			power: power,
			shot_timer_start : 0,
			keyboard: keyboard,
			state: State::Stand,
			last_state: State::Stand
		}
	}
}

impl Default for InputComponent {
	fn default() -> Self {
		InputComponent::new(0.0, false)
	}
}
