use generic_static::StaticTypeMap;
use once_cell::sync::OnceCell;
use fixedbitset::FixedBitSet;
use std::{any::Any, collections::{HashMap, HashSet, hash_set}, sync::{Arc, RwLock, Weak, atomic::{AtomicUsize, Ordering}}};

const ECS_MAX_COMPONENTS: usize = 100;
const ECS_MAX_ENTITIES: usize = 10000;

type ComponentId = usize;
pub type EntityId = usize;

pub trait ComponentIdMaker {
	fn unique_id() -> ComponentId;
}

impl<T> ComponentIdMaker for T {
	fn unique_id() -> ComponentId {
		static COMPONENT_COUNTER : AtomicUsize = AtomicUsize::new(0);
		let result = COMPONENT_COUNTER.load(Ordering::SeqCst) as ComponentId;
		COMPONENT_COUNTER.fetch_add(1, Ordering::SeqCst);
		result
	}
}

pub fn build_component_id<T: ComponentIdMaker + 'static>() -> &'static ComponentId {
	static VALUE: OnceCell<StaticTypeMap<ComponentId>> = OnceCell::new();
	let map = VALUE.get_or_init(|| StaticTypeMap::new());

	map.call_once::<T, _>(||{
		T::unique_id()
	})
}

pub trait Component: Any + Sized { }
impl<T: Any> Component for T {}

pub trait Holder {
	fn is_init(&self) -> bool;
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Component> Holder for ComponentHolder<T> {
	fn as_any(&self) -> &dyn Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
	fn is_init(&self) -> bool {
		self.init
	}
}

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

	fn new() -> Self {
		let mut result = ComponentHolder::<T> {
			components: Vec::with_capacity(ECS_MAX_ENTITIES),
			init: true
		};
		result.components.resize_with(ECS_MAX_ENTITIES, T::default);
		result
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

trait EventObserver<T> {
	fn on_event_mut(&mut self, data: &T);
}

type Observer<T> = RwLock<dyn EventObserver<T>>;
struct EventBusBase<T> {
	subscribers: Vec<Weak<Observer<T>>>
}

impl<T> EventBusBase<T> {
	pub fn new() -> Self {
		EventBusBase {
			subscribers: Vec::new()
		}
	}
}

trait EventBus<T> {
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

impl EventBus<ComponentEvent> for World {
	fn register(&mut self, observer: Arc<Observer<ComponentEvent>>) {
		self.event_bus.register(observer);
	}

	fn notify(&self, data: &ComponentEvent) {
		self.event_bus.notify(data);
	}
}

struct ComponentEvent {
	component_id: ComponentId,
	entity_id: EntityId,
	event_type: i16,
	entity_mask: Weak<RwLock<FixedBitSet>>
}

pub struct SystemHolder {
	runnables: Vec<Box<dyn Runnable>>
}

impl SystemHolder {
	pub fn new() -> Self {
		SystemHolder {
			runnables: Vec::new()
		}
	}

	pub fn add_system<T: Runnable + SystemNewable<T, Args> + SystemComponents + 'static, Args>(&mut self, world: &mut World, args: Args) {
		let system = System::new::<T::Components>();
		world.register(system.clone());
		world.register_system_base(system.clone());
		self.runnables.push(Box::new(T::new(system.clone(), args)));
	}

	pub fn update<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>) {
		game_services.get_world_mut().update();
		for system in self.runnables.iter_mut() {
			system.run(game_services);
		}
	}
}

pub struct World {
	entities: HashSet<EntityId>,
	entities_mask: Vec<Arc<RwLock<FixedBitSet>>>,
	dead_entities: Vec<EntityId>,
	holders: Vec<Box<dyn Holder>>,
	event_bus: EventBusBase<ComponentEvent>,
	components_remove_queue: HashSet<(EntityId, ComponentId)>,
	entities_remove_queue: HashSet<EntityId>,
	events: Vec<ComponentEvent>,
	bases: HashMap<u64, Arc<RwLock<System>>>
}

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
		T::Components::set_id(&mut imask);
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

	pub fn is_alive(&self, id: &EntityId) -> bool {
		self.entities.contains(id)
	}

	pub fn add_component<T: Component + Default>(&mut self, entity_id: &EntityId, component: T) {
		let holder = self.holder_mut::<T>();
		let component_id = *build_component_id::<T>();
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
			self.entities_mask[*entity_id] = Arc::new(RwLock::new(FixedBitSet::new()));
			self.entities.remove(entity_id);
			self.dead_entities.push(*entity_id);
			self.destruct_component(entity_id, &usize::MAX);
		}
		self.entities_remove_queue.clear();
	}

	pub fn remove_component<T: Component + 'static>(&mut self, entity_id: &EntityId) {
		if self.has_component::<T>(entity_id) {
			let component_id = build_component_id::<T>();
			self.components_remove_queue.insert((*entity_id, *component_id));
		}
	}

	pub fn has_component<T: Component>(&self, entity_id: &EntityId) -> bool {
		let component_id = build_component_id::<T>();
		self.entities_mask[*entity_id].read().unwrap()[*component_id]
	}

	pub fn get_component<T: Component + Default>(&self, entity_id: &EntityId) -> Option<&T> {
		if ! self.has_component::<T>(entity_id) {
			Option::None
		} else {
			let holder = self.holder::<T>();
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
			let holder = self.holder_mut::<T>();
			Option::Some(holder.get_component_mut(entity_id))
		}
	}

	fn lazy_init_holder<T: Component + Default>(&mut self) -> ComponentId {
		let component_id = *build_component_id::<T>();
		let less = component_id >= self.holders.len();
		if ! less {
			if !self.holders[component_id].is_init() {
				self.holders[component_id] = Box::new(ComponentHolder::<T>::new());
			}
			return component_id;
		}

		while component_id > self.holders.len() {
			//println!("COMPENSATE HOLDER {}", self.holders.len());
			self.holders.push(Box::new(ComponentHolder::<T>::default()));
		}
		self.holders.push(Box::new(ComponentHolder::<T>::new()));
		//println!("INIT HOLDER {}", component_id);
		component_id
	}

	fn get_holder<T: Component + Default>(&self) -> Option<ComponentId> {
		let component_id = *build_component_id::<T>();
		if component_id < self.holders.len() && self.holders[component_id].is_init() {
			return Some(component_id);
		}
		return None;
	}

	pub fn holder<T: Component + Default>(&self) -> Option<&ComponentHolder<T>> {
		let component_id = self.get_holder::<T>();
		if component_id.is_none() {
			return None;
		}
		let result = self.holders.get(component_id.unwrap());
		let any_box = result.unwrap().as_any();
		Some(any_box.downcast_ref::<ComponentHolder<T>>().unwrap())
	}

	pub fn holder_mut<T: Component + Default>(&mut self) -> &mut ComponentHolder<T> {
		let component_id = self.lazy_init_holder::<T>();
		let result = self.holders.get_mut(component_id);
		let any_box = result.unwrap().as_any_mut();
		any_box.downcast_mut::<ComponentHolder<T>>().unwrap()
	}

}

use tuple_list::{TupleList};

use super::common::GameServices;

pub trait ComponentMaskSetBit {
	fn set_bitset(bitset: &mut FixedBitSet);
	fn set_id(imask: &mut u64);
}

impl ComponentMaskSetBit for () {
	fn set_bitset(_bitset: &mut FixedBitSet) {}
	fn set_id(_imask: &mut u64) {}
}

impl<Head, Tail> ComponentMaskSetBit for (Head, Tail) where
	Head: 'static,
	Tail: ComponentMaskSetBit + TupleList,
{
	fn set_bitset(bitset: &mut FixedBitSet) {
		let component_id = build_component_id::<Head>();
		bitset.set(*component_id, true);
		Tail::set_bitset(bitset);
	}

	fn set_id(imask: &mut u64) {
		let component_id = build_component_id::<Head>();
		*imask |= 2u64.pow(*component_id as u32);
		Tail::set_id(imask)
	}
}

pub struct System {
	mask: FixedBitSet,
	entities: HashSet<EntityId>,
	id: u64
}

pub trait Runnable {
	fn run<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all, 'l>);
}

pub trait SystemComponents {
	type Components: ComponentMaskSetBit + TupleList;
}

pub trait SystemNewable<T, Args> {
	fn new(base: Arc<RwLock<System>>, args: Args) -> T;
}

impl System {
	pub fn new<Components: ComponentMaskSetBit + TupleList>() -> Arc<RwLock<Self>> {
		let mut system = System {
			mask: FixedBitSet::with_capacity(ECS_MAX_COMPONENTS),
			entities: HashSet::new(),
			id: 0
		};
		Components::set_bitset(&mut system.mask);
		Components::set_id(&mut system.id);
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
			if (&self.mask & &data.entity_mask.upgrade().unwrap().read().unwrap()) == self.mask {
				//println!("ENTITY {} ADDED TO SYSTEM", data.entity_id);
				self.entities.insert(data.entity_id);
			} else {
				//println!("ENTITY {} IS MISSING SOME BITS", data.entity_id);
			}
			//println!("SYSTEM : {}\nENTITY : {}", self.mask, data.entity_mask.upgrade().unwrap().read().unwrap());
		} else if data.event_type == 1 && self.entities.contains(&data.entity_id) {
			//println!("ENTITY {} REMOVED FROM SYSTEM", data.entity_id);
			self.entities.remove(&data.entity_id);
		}
	}
}
