use std::sync::{Arc, RwLock, Weak};


pub trait EventObserver<T> {
	fn on_event_mut(&mut self, data: &T);
}

pub type Observer<T> = RwLock<dyn EventObserver<T>>;
pub struct EventBusBase<T> {
	subscribers: Vec<Weak<Observer<T>>>
}

impl<T> EventBusBase<T> {
	pub fn new() -> Self {
		EventBusBase {
			subscribers: Vec::new()
		}
	}
}

pub trait EventBus<T> {
	fn register(&mut self, observer: Arc<Observer<T>>);
	fn notify(&self, data: &T);
}

impl<T> EventBus<T> for EventBusBase<T> {
	fn register(&mut self, observer: Arc<Observer<T>>) {
		self.subscribers.push(Arc::downgrade(&observer));
	}

	fn notify(&self, data: &T) {
		for obs in self.subscribers.iter() {
			if let Some(listener_rc) = obs.upgrade() {
				let mut listener = listener_rc.write().unwrap();
				listener.on_event_mut(&data);
			} /* else {
				// SOME LISTENERS ARE NOW DEAD, MUST CLEANUP
			} */
		}
	}
}
