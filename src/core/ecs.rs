use fixedbitset::FixedBitSet;
use once_cell::sync::OnceCell;
use std::{any::Any, borrow::Borrow, collections::{HashMap, HashSet, hash_set}, sync::{Arc, RwLock, Weak, atomic::AtomicUsize}};

const ECS_MAX_COMPONENTS: usize = 100;
const ECS_MAX_ENTITIES: usize = 10000;

type ComponentId = usize;
pub type EntityId = usize;


pub trait Component: Any + Sized { }
impl<T: Any> Component for T {}

impl<T: Component + Default> Initable for ComponentHolder<T> {
	fn is_init(&self) -> bool {
		self.init
	}

	fn init(&mut self) {
		self.components = Vec::with_capacity(ECS_MAX_ENTITIES);
		self.components.resize_with(ECS_MAX_ENTITIES, T::default);
		self.init = true;
	}
}

impl<T: Component + Default> Holder for ComponentHolder<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

impl<T: Initable + Holder> InitableHolder for T {}

pub struct ComponentHolder<T: Component> {
	components: Vec<T>,
	init: bool
}

impl<T: Component + Default> ComponentHolder<T> {
	pub fn add_component(self: &mut Self, entity_id: &EntityId, component: T) {
		assert!(*entity_id < ECS_MAX_ENTITIES && self.init);
		self.components[*entity_id] = component;
	}

	pub fn get_component(self: &Self, entity_id: &EntityId) -> &T {
		assert!(*entity_id < ECS_MAX_ENTITIES && self.init);
		&self.components[*entity_id]
	}

	pub fn get_component_mut(self: &mut Self, entity_id: &EntityId) -> &mut T {
		&mut self.components[*entity_id]
	}

}

impl<T: Component + Default> Default for ComponentHolder<T> {
	fn default() -> Self {
		ComponentHolder {
			components: Vec::new(),
			init: false
		}
	}
}

impl EventBus<ComponentEvent> for World {
	fn register(&mut self, observer: Arc<Observer<ComponentEvent>>) {
		self.event_bus.register(observer);
	}

	fn notify(&self, data: &ComponentEvent) {
		self.event_bus.notify(data);
	}

	fn unregister(&mut self, observer: Arc<Observer<ComponentEvent>>) {
		self.event_bus.unregister(observer)
	}
}

struct ComponentEvent {
	component_id: ComponentId,
	entity_id: EntityId,
	event_type: i16,
	entity_mask: Weak<RwLock<FixedBitSet>>
}


struct SystemHandle {
	system: Option<Box<dyn Runnable>>,
	id: Option<*const dyn Runnable>,
	alive: bool
}
pub struct SystemHolder {
	runnables: Vec<Box<dyn Runnable>>,
	all: HashMap<u64, SystemHandle>
}

impl SystemHolder {
	pub fn new() -> Self {
		SystemHolder {
			runnables: Vec::new(),
			all: HashMap::new()
		}
	}

	pub fn add_system<T: Runnable + SystemNewable<T, Args> + SystemComponents + 'static, Args>(&mut self, world: &mut World, args: Args) {
		let base = System::new::<T::Components>();
		world.register_system_base(base.clone());
		world.register(base.clone());
		let id= * meta::numeric_type_id::<T>(&states::SYSTEM_ID_COUNTER) as u64;
		self.all.insert(id, SystemHandle { id: None, system: Some(Box::new(T::new(base.clone(), args))), alive: false });
	}

	pub fn update<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		game_services.get_world_mut().update();
		for system in self.runnables.iter_mut() {
			system.run(game_services);
		}
	}

	pub fn enable_system(&mut self, _world: &mut World, system_id: u64) {
		if let Some(system) = self.all.get_mut(&system_id) {
			if ! system.alive {
				let t = system.system.as_ref().unwrap();
				system.id = Some(t.borrow() as *const dyn Runnable);
				self.runnables.push(system.system.take().unwrap());
				system.alive = true;
			}
		}
	}

	pub fn disable_system(&mut self, _world: &mut World, system_id: u64) {
		if let Some(system) = self.all.get_mut(&system_id) {
			if system.alive {
				let mut index = 0;
				for (i, run) in self.runnables.iter().enumerate() {
					let handle_ptr: *const dyn Runnable = run.as_ref();
					if handle_ptr == system.id.unwrap() {
						index = i;
						break;
					}
				}
				system.system = Some(self.runnables.remove(index));
				system.id = None;
				system.alive = false;
			}
		}
	}
}

pub struct World {
	entities: HashSet<EntityId>,
	entities_mask: Vec<Arc<RwLock<FixedBitSet>>>,
	dead_entities: Vec<EntityId>,
	holders: Vec<Box<dyn InitableHolder>>,
	event_bus: EventBusBase<ComponentEvent>,
	components_remove_queue: HashSet<(EntityId, ComponentId)>,
	entities_remove_queue: HashSet<EntityId>,
	events: Vec<ComponentEvent>,
	bases: HashMap<u64, Arc<RwLock<System>>>
}

static COMPONENT_ID_COUNTER: IdCounter = IdCounter { cell: OnceCell::new(), atomic: AtomicUsize::new(0) };

impl World {
	pub fn new() -> Self {
		let mut world = World {
			entities: HashSet::new(),
			dead_entities: Vec::new(),
			entities_mask: Vec::with_capacity(ECS_MAX_ENTITIES),
			holders: Vec::with_capacity(ECS_MAX_COMPONENTS),
			event_bus: EventBusBase::new(),
			components_remove_queue: HashSet::new(),
			entities_remove_queue: HashSet::new(),
			events: Vec::new(),
			bases: HashMap::new()
		};
		world.entities_mask.resize(ECS_MAX_ENTITIES, Arc::new(RwLock::new(FixedBitSet::new())));
		world
	}

	pub fn get_system_base<T: SystemComponents + 'static>(&self) -> Option<Weak<RwLock<System>>>{
		let mut imask: u64 = 0;
		T::Components::set_id(&COMPONENT_ID_COUNTER, &mut imask);
		let result = self.bases.get(&imask);
		if result.is_none() {
			None
		} else {
			Some(Arc::downgrade(result.unwrap()))
		}
	}

	fn register_system_base(&mut self, system :Arc<RwLock<System>>) {
		self.bases.insert(system.read().as_ref().unwrap().id, system.clone());
	}

	pub fn create_entity(&mut self) -> EntityId {
		let id;
		if self.dead_entities.is_empty() {
			id = self.entities.len();
		} else {
			id = *self.dead_entities.last().unwrap();
			self.dead_entities.pop();
		}
		self.entities_mask[id] = Arc::new(RwLock::new(FixedBitSet::with_capacity(ECS_MAX_COMPONENTS)));
		self.entities.insert(id);
		id
	}

	pub fn remove_entity(&mut self, id: &EntityId) {
		self.entities_remove_queue.insert(*id);
	}

	pub fn reset(&mut self) {
		self.entities_remove_queue.clear();
		self.components_remove_queue.clear();
		self.events.clear();
		for entity_id in &self.entities {
			self.entities_mask[*entity_id] = Arc::new(RwLock::new(FixedBitSet::new()));
			self.destruct_component(entity_id, &usize::MAX);
		}
		self.entities.clear();
		self.dead_entities.clear();
	}

	pub fn is_alive(&self, id: &EntityId) -> bool {
		self.entities.contains(id)
	}

	pub fn add_component<T: Component + Default>(&mut self, entity_id: &EntityId, component: T) {
		let holder = meta::holder_mut::<T, ComponentHolder<T>>(&COMPONENT_ID_COUNTER, &mut self.holders);
		let component_id = *meta::numeric_type_id::<T>(&COMPONENT_ID_COUNTER);
		//println!("ID : {}", component_id);
		holder.add_component(entity_id, component);
		self.entities_mask[*entity_id].write().unwrap().set(component_id, true);
		let event = ComponentEvent {
			component_id: component_id,
			entity_id: *entity_id,
			entity_mask : Arc::downgrade(&self.entities_mask[*entity_id]),
			event_type: 0
		};
		self.events.push(event);
	}

	fn destruct_component(&self, entity_id: &EntityId, component_id: &ComponentId) {
		if *component_id < usize::MAX {
			self.entities_mask[*entity_id].write().unwrap().set(*component_id, false);
		}
		let event = ComponentEvent {
			component_id: *component_id,
			entity_id: *entity_id,
			entity_mask : Arc::downgrade(&self.entities_mask[*entity_id]),
			event_type: 1
		};
		self.notify(&event);
	}

	pub fn update(&mut self) {
		for components_remove_pair in self.components_remove_queue.iter() {
			self.destruct_component(&components_remove_pair.0, &components_remove_pair.1);
		}
		self.components_remove_queue.clear();

		for event in self.events.iter() {
			self.notify(&event);
		}
		self.events.clear();

		for entity_id in self.entities_remove_queue.iter() {
			if self.entities.contains(entity_id) {
				self.entities_mask[*entity_id] = Arc::new(RwLock::new(FixedBitSet::new()));
				self.entities.remove(entity_id);
				self.dead_entities.push(*entity_id);
				self.destruct_component(entity_id, &usize::MAX);
			}
		}
		self.entities_remove_queue.clear();
	}

	pub fn remove_component<T: Component + 'static>(&mut self, entity_id: &EntityId) {
		if self.has_component::<T>(entity_id) {
			let component_id = meta::numeric_type_id::<T>(&COMPONENT_ID_COUNTER);
			self.components_remove_queue.insert((*entity_id, *component_id));
		}
	}

	pub fn has_component<T: Component>(&self, entity_id: &EntityId) -> bool {
		let component_id = meta::numeric_type_id::<T>(&COMPONENT_ID_COUNTER);
		self.entities_mask[*entity_id].read().unwrap()[*component_id]
	}

	pub fn get_component<T: Component + Default>(&self, entity_id: &EntityId) -> Option<&T> {
		if ! self.has_component::<T>(entity_id) {
			Option::None
		} else {
			let holder = meta::holder::<T, ComponentHolder<T>>(&COMPONENT_ID_COUNTER, &self.holders);
			if holder.is_none() {
				return None;
			}
			Option::Some(holder.unwrap().get_component(entity_id))
		}
	}

	pub fn get_component_mut<T: Component + Default>(&mut self, entity_id: &EntityId) -> Option<&mut T> {
		if ! self.has_component::<T>(entity_id) {
			Option::None
		} else {
			let holder = meta::holder_mut::<T, ComponentHolder<T>>(&COMPONENT_ID_COUNTER, &mut self.holders);
			Option::Some(holder.get_component_mut(entity_id))
		}
	}

}

use tuple_list::{TupleList};

use super::{common::GameServices, events::{EventBus, EventBusBase, EventObserver, Observer}, meta::{self, Holder, IdCounter, Initable, InitableHolder, TypeMaskSetBit}, states};

pub struct System {
	mask: FixedBitSet,
	entities: HashSet<EntityId>,
	id: u64
}

pub trait Runnable {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>);
}

pub type SharedRunnable =  Arc<RwLock<dyn Runnable>>;
pub type WeakRunnable =  Weak<RwLock<dyn Runnable>>;
pub type SharedGRunnable<T> = Arc<RwLock<T>>;

pub fn make_shared_runnable<T: Runnable + 'static>(runnable: SharedGRunnable<T>) -> SharedRunnable {
	runnable
}
pub trait SystemComponents {
	type Components: TypeMaskSetBit + TupleList;
}

pub trait SystemNewable<T, Args> {
	fn new(base: Arc<RwLock<System>>, args: Args) -> T;
}

impl System {
	pub fn new<Components: TypeMaskSetBit + TupleList>() -> Arc<RwLock<Self>> {
		let mut system = System {
			mask: FixedBitSet::with_capacity(ECS_MAX_COMPONENTS),
			entities: HashSet::new(),
			id: 0
		};
		Components::set_bitset(&COMPONENT_ID_COUNTER, &mut system.mask);
		Components::set_id(&COMPONENT_ID_COUNTER, &mut system.id);
		println!("MASK {}, ID {}", system.mask, system.id);
		Arc::new(RwLock::new(system))
	}

	pub fn iter_entities(&self) -> hash_set::Iter<EntityId> {
		self.entities.iter()
	}

	pub fn len_entities(&self) -> usize {
		self.entities.len()
	}
}

impl EventObserver<ComponentEvent> for System {
	fn on_event_mut(&mut self, data: &ComponentEvent) {
		let contained = self.mask.contains(data.component_id);
		if data.event_type == 0 && contained && !self.entities.contains(&data.entity_id) {
			if let Some(mask) = data.entity_mask.upgrade() {
				if (&self.mask & &mask.read().unwrap()) == self.mask {
					//println!("ENTITY {} ADDED TO SYSTEM", data.entity_id);
					self.entities.insert(data.entity_id);
				} else {
					//println!("ENTITY {} IS MISSING SOME BITS", data.entity_id);
				}
			}
			//println!("SYSTEM : {}\nENTITY : {}", self.mask, data.entity_mask.upgrade().unwrap().read().unwrap());
		} else if data.event_type == 1 && self.entities.contains(&data.entity_id) {
			//println!("ENTITY {} REMOVED FROM SYSTEM", data.entity_id);
			self.entities.remove(&data.entity_id);
		}
	}
}
