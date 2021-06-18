use crate::{core::animation::Animation};


pub struct AnimationComponent {
	pub all: Vec<Animation>,
	pub current: usize,
	pub next: Option<usize>
}

impl AnimationComponent {
	pub fn new() -> Self {
		AnimationComponent{
			all: Vec::new(),
			current: 0,
			next: None
		}
	}

	pub fn set(&mut self, animations: Vec<Animation>) {
		self.all = animations;
	}

	pub fn update(&mut self) -> (bool, u16) {
		if self.next.is_some() && self.all[self.current].is_done() {
			self.current = self.next.take().unwrap();
		}
		self.all[self.current].update(0)
	}

	pub fn next(&mut self, next: usize) {
		if ! self.all[self.current].is_started() {
			self.current = next;
			self.all[self.current].start();
		} else {
			self.next = Some(next);
			let last_offset = self.all[self.current].get_offset();
			let all = &mut self.all[self.next.unwrap()];
			all.reset();
			all.current_offset(last_offset);
			all.start();
		}
	}

	pub fn get_origin(&self) -> usize {
		self.all[self.current].get_origin()
	}

	pub fn get_offset(&self) -> usize {
		self.all[self.current].get_offset()
	}
}

impl Default for AnimationComponent {
	fn default() -> Self {
		AnimationComponent::new()
	}
}
