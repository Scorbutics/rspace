use crate::{core::animation::Animation};


pub struct AnimationComponent {
	pub all: Vec<Animation>,
	pub current: usize
}

impl AnimationComponent {
	pub fn new() -> Self {
		AnimationComponent{
			all: Vec::new(),
			current: 0
		}
	}

	pub fn set(&mut self, animations: Vec<Animation>) {
		self.all = animations;
	}

	pub fn update(&mut self) -> (bool, u16) {
		self.all[self.current].update(0)
	}

	pub fn reset(&mut self)  {
		self.all[self.current].reset()
	}

	pub fn start(&mut self) -> &mut Animation {
		self.all[self.current].start()
	}

	pub fn current_offset(&mut self, offset: usize) {
		self.all[self.current].current_offset(offset)
	}

	pub fn get_offset(&self) -> usize {
		self.all[self.current].get_offset()
	}

	pub fn get_origin(&self) -> usize {
		self.all[self.current].get_origin()
	}
}

impl Default for AnimationComponent {
	fn default() -> Self {
		AnimationComponent::new()
	}
}
