use std::{cell::RefCell, rc::Rc};

use crate::core::common;

#[derive(Clone)]
pub struct Animation {
	origin: usize,
	offset: usize,
	offset_start: usize,
	frames: usize,
	step: i16,
	delay_ms: u64,
	start_ms: u64,
	count: i32,
	count_start: i32,
	started: bool,
	next: Option<Rc<RefCell<Animation>>>
}

enum AnimationOutOfBound {
	None,
	Top,
	Bottom
}

impl Animation {
	pub fn new(origin: usize) -> Self {
		Animation {
			origin: origin,
			offset: 0,
			offset_start: 0,
			frames: 0,
			step: 1,
			delay_ms: 0,
			start_ms: 0,
			count: 0,
			count_start: 0,
			started: false,
			next: None
		}
	}

	pub fn has_next(&self) -> bool {
		self.next.is_some()
	}

	pub fn frames(&mut self, frames: usize) -> &mut Self  {
		self.frames = frames;
		self
	}

	pub fn time(&mut self, delay_ms: u64) -> &mut Self {
		self.delay_ms = delay_ms;
		self
	}

	pub fn count(&mut self, count: i32) -> &mut Self {
		self.count = count;
		self.count_start = count;
		self
	}

	pub fn is_started(&self) -> bool {
		self.started
	}

	pub fn reset(&mut self) {
		self.count = self.count_start;
		self.offset = self.offset_start;
		//println!("reset to {} count & {} offset", self.count, self.offset);
		if self.next.is_some() { self.next.as_mut().unwrap().borrow_mut().reset(); }
	}

	pub fn reverse(&mut self) -> &mut Self {
		self.step *= -1;
		self
	}

	pub fn origin(&mut self, origin: usize) -> &mut Self {
		self.origin = origin;
		self
	}

	pub fn then(&mut self, next: &Animation) -> &mut Self {
		self.next = Some(Rc::new(RefCell::new(next.clone())));
		self
	}

	pub fn offset(&mut self, offset: usize) -> &mut Self {
		self.offset = offset;
		self.offset_start = offset;
		self
	}

	pub fn current_offset(&mut self, offset: usize) {
		self.offset = offset;
	}

	pub fn get_offset(&self) -> usize {
		if self.count == 0 && self.next.is_some()  {
			self.next.as_ref().unwrap().borrow().get_offset()
		} else {
			self.offset
		}
	}

	pub fn get_origin(&self) -> usize {
		if self.count == 0 && self.next.is_some()  {
			self.next.as_ref().unwrap().borrow().get_origin()
		} else {
			self.origin
		}
	}

	pub fn start(&mut self) -> &mut Self {
		self.started = true;
		if self.next.is_some() { self.next.as_mut().unwrap().borrow_mut().start(); }
		self
	}

	pub fn pause(&mut self) -> &mut Self {
		self.started = false;
		if self.next.is_some() { self.next.as_mut().unwrap().borrow_mut().pause(); }
		self
	}

	pub fn is_done(&self) -> bool {
		self.count == 0 && (self.next.is_none() || self.next.as_ref().unwrap().borrow().is_done())
	}

	fn next_step_out_of_bounds(&self) -> AnimationOutOfBound {
		let next_step = self.get_offset() as i16 + self.step;
		if next_step >= self.frames as i16 {
			AnimationOutOfBound::Top
		} else if next_step < 0 {
			AnimationOutOfBound::Bottom
		} else {
			AnimationOutOfBound::None
		}
	}

	pub fn update(&mut self, i: u16) -> (bool, u16) {
		if ! self.started { return (false, i); }
		if self.count == 0 {
			if self.next.is_some() {
				return self.next.as_mut().unwrap().borrow_mut().update(i + 1);
			}
			return (false, i);
		}
		let off = self.get_offset();
		//println!("{} frame {}/{}, step {}, count {}", i, off, self.frames - 1, self.step, self.count);

		let delay_off = common::current_time_ms() - self.start_ms >= self.delay_ms;
		if delay_off {
			let sprite_index = off;
			let next_index = match self.next_step_out_of_bounds() {
				AnimationOutOfBound::Top => {
					self.count -= 1;
					self.frames as i16 - 1
				},
				AnimationOutOfBound::Bottom => {
					self.count -= 1;
					0
				},
				AnimationOutOfBound::None => sprite_index as i16 + self.step
			};

			self.offset = next_index as usize;
			self.start_ms = common::current_time_ms();
		}
		return (true, i);
	}
}

/*
fn test() {
	let mut all = AllAnimations::new();

	println!("Stand (no action) --------");

	change_animation_depending_on_moving(&mut all, State::Stand, State::Stand);
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();

	println!("Stand => MoveRight --------");

	change_animation_depending_on_moving(&mut all, State::Stand, State::MoveRight);
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();

	println!("MoveRight => MoveLeft --------");

	change_animation_depending_on_moving(&mut all, State::MoveRight, State::MoveLeft);
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();

	println!("MoveLeft => Stand --------");

	change_animation_depending_on_moving(&mut all, State::MoveLeft, State::Stand);
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();

	println!("Stand => MoveLeft --------");

	change_animation_depending_on_moving(&mut all, State::Stand, State::MoveLeft);
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();

	println!("MoveLeft => MoveRight => Stand => MoveLeft --------");

	change_animation_depending_on_moving(&mut all, State::MoveLeft, State::MoveRight);
	all.update();
	change_animation_depending_on_moving(&mut all, State::MoveRight, State::Stand);
	all.update();
	change_animation_depending_on_moving(&mut all, State::Stand, State::MoveLeft);
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
	all.update();
}
*/