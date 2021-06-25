use std::{any::Any, sync::{Arc, RwLock, Weak, atomic::AtomicUsize}};

use once_cell::sync::OnceCell;

use super::meta::{self, Holder, IdCounter, Initable, InitableHolder};


pub trait EventObserver<T> {
	fn on_event_mut(&mut self, data: &T);
}

pub type Observer<T> = RwLock<dyn EventObserver<T>>;
pub struct EventBusBase<T: 'static> {
	subscribers: Vec<Weak<Observer<T>>>,
	init: bool
}

impl<T> EventBusBase<T> {
	pub fn new() -> Self {
		EventBusBase { subscribers: Vec::new(), init: true }
	}
}

impl<T> Default for EventBusBase<T> {
	fn default() -> Self {
		EventBusBase { subscribers: Vec::new(), init: false }
	}
}

impl<T> Initable for EventBusBase<T> {
	fn is_init(&self) -> bool {
		self.init
	}

	fn init(&mut self) {
		self.init = true;
	}
}

impl<T> Holder for EventBusBase<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

pub trait EventBus<T> {
	fn register(&mut self, observer: Arc<Observer<T>>);
	fn unregister(&mut self, observer: Arc<Observer<T>>);
	fn notify(&self, data: &T);
}

impl<T> EventBus<T> for EventBusBase<T> {
	fn register(&mut self, observer: Arc<Observer<T>>) {
		self.subscribers.push(Arc::downgrade(&observer));
	}

	fn unregister(&mut self, observer: Arc<Observer<T>>) {
		self.subscribers.retain(|el| el.as_ptr() != &*observer );
	}

	fn notify(&self, data: &T) {
		let mut i = 0;
		while i < self.subscribers.len() {
			let obs = &self.subscribers[i];
			if let Some(listener_rc) = obs.upgrade() {
				let mut listener = listener_rc.write().unwrap();
				listener.on_event_mut(&data);
				i += 1;
			}
			// TODO : must be mutable...
			/*else {
				self.subscribers.remove(i);
			}*/
		}
	}
}

static EVENT_ID_COUNTER: IdCounter = IdCounter { cell: OnceCell::new(), atomic: AtomicUsize::new(0) };

pub struct EventDispatcher {
	holders: Vec<Box<dyn InitableHolder>>
}

pub trait Event: Any + Sized { }
impl<T: Any> Event for T {}

impl EventDispatcher {
	pub fn new() -> Self { EventDispatcher { holders: Vec::new() } }

	pub fn register<T: 'static>(&mut self, observer: Arc<Observer<T>>) {
		let holder = meta::holder_mut::<T, EventBusBase<T>>(&EVENT_ID_COUNTER, &mut self.holders);
		holder.register(observer)
	}

	pub fn notify<T: 'static>(&self, data: &T) {
		let holder = meta::holder::<T, EventBusBase<T>>(&EVENT_ID_COUNTER, &self.holders).unwrap();
		holder.notify(data)
	}
}
