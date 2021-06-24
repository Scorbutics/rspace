use std::{any::{Any}, sync::atomic::{AtomicUsize, Ordering}};

use fixedbitset::FixedBitSet;
use generic_static::StaticTypeMap;
use once_cell::sync::OnceCell;
use tuple_list::TupleList;

pub trait Initable {
	fn is_init(&self) -> bool;
	fn init(&mut self);
}

pub trait Holder {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait InitableHolder: Initable + Holder {}

pub trait TypeIdMaker {
	fn unique_id(atomic: &'static AtomicUsize) -> usize;
}

impl<T> TypeIdMaker for T {
	fn unique_id(atomic: &'static AtomicUsize) -> usize {
		let result = atomic.load(Ordering::SeqCst) as usize;
		//println!("Generating id {}", result);
		atomic.fetch_add(1, Ordering::SeqCst);
		result
	}
}

pub struct IdCounter {
	pub cell: OnceCell<StaticTypeMap<usize>>,
	pub atomic: AtomicUsize
}

pub fn numeric_type_id<T: TypeIdMaker + 'static>(unique_counter: &'static IdCounter) -> &'static usize {
	let map = unique_counter.cell.get_or_init(|| StaticTypeMap::new());

	map.call_once::<T, _>(||{
		T::unique_id(&unique_counter.atomic)
	})
}

fn lazy_init_holder<T: 'static, X: InitableHolder + Default + 'static>(unique_counter: &'static IdCounter, holders : &mut Vec<Box<dyn InitableHolder>>) -> usize {
	let component_id = * numeric_type_id::<T>(unique_counter);
	let less = component_id >= holders.len();
	if ! less {
		if !holders[component_id].is_init() {
			//println!("INIT HOLDER {} / {} previously un inited", component_id, holders.len());
			holders[component_id] = Box::new(X::default());
			holders[component_id].init();
		}
		return component_id;
	}

	while component_id > holders.len() {
		//println!("COMPENSATE HOLDER {}", holders.len());
		holders.push(Box::new(X::default()));
	}
	let mut holder = X::default();
	holder.init();
	holders.push(Box::new(holder));
	//println!("INIT HOLDER {} / {}", component_id, holders.len());
	component_id
}

fn get_holder<T: 'static>(unique_counter: &'static IdCounter, holders : &Vec<Box<dyn InitableHolder>>) -> Option<usize> {
	let component_id = * numeric_type_id::<T>(unique_counter);
	if component_id < holders.len() && holders[component_id].is_init() {
		return Some(component_id);
	}
	return None;
}

pub fn holder<'l, T: 'static, X: 'static>(unique_counter: &'static IdCounter, holders : &'l Vec<Box<dyn InitableHolder>>) -> Option<&'l X> {
	let component_id = get_holder::<T>(unique_counter, holders);
	if component_id.is_none() {
		return None;
	}
	let result = holders.get(component_id.unwrap()).unwrap();
	let any_box = result.as_any();
	Some(any_box.downcast_ref::<X>().unwrap())
}

pub fn holder_mut<'l, T: 'static, X: InitableHolder + Default + 'static>(unique_counter: &'static IdCounter, holders: &'l mut Vec<Box<dyn InitableHolder>>) -> &'l mut X {
	let component_id = lazy_init_holder::<T, X>(unique_counter, holders);
	let result = holders.get_mut(component_id).unwrap();
	let any_box = result.as_any_mut();
	any_box.downcast_mut::<X>().unwrap()
}


pub trait TypeMaskSetBit {
	fn set_bitset(unique_counter: &'static IdCounter, bitset: &mut FixedBitSet);
	fn set_id(unique_counter: &'static IdCounter, imask: &mut u64);
}

impl TypeMaskSetBit for () {
	fn set_bitset(_unique_counter: &'static IdCounter, _bitset: &mut FixedBitSet) {}
	fn set_id(_unique_counter: &'static IdCounter, _imask: &mut u64) {}
}

impl<Head, Tail> TypeMaskSetBit for (Head, Tail) where
	Head: 'static,
	Tail: TypeMaskSetBit + TupleList,
{
	fn set_bitset(unique_counter: &'static IdCounter, bitset: &mut FixedBitSet) {
		let component_id = numeric_type_id::<Head>(unique_counter);
		bitset.set(*component_id, true);
		Tail::set_bitset(unique_counter, bitset);
	}

	fn set_id(unique_counter: &'static IdCounter, imask: &mut u64) {
		let component_id = numeric_type_id::<Head>(unique_counter);
		*imask |= 2u64.pow(*component_id as u32);
		Tail::set_id(unique_counter, imask)
	}
}
