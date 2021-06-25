/*
	CREDITS TO RUST-SDL2 EXAMPLE
	FOR THIS CODE
*/

use std::{borrow::Borrow, collections::HashMap, hash::Hash, rc::Rc};

use sdl2::image::LoadTexture;

use sdl2::{render::{Texture, TextureCreator}, ttf::{Font, Sdl2TtfContext}};
// Generic struct to cache any resource loaded by a ResourceLoader
pub struct ResourceManager<'l, K, R, L>
where
	K: Hash + Eq,
	L: ResourceLoader<'l, R>,
{
	pub loader: &'l L,
	cache: Vec<Rc<R>>,
	index_cache: HashMap<K, usize>,
	unique_cache: Vec<Option<Box<R>>>
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L>
where
	K: Hash + Eq,
	L: ResourceLoader<'l, R>,
{
	pub fn new(loader: &'l L) -> Self {
		ResourceManager {
			cache: Vec::new(),
			loader: loader,
			index_cache: HashMap::new(),
			unique_cache: Vec::new()
		}
	}

	// Generics magic to allow a HashMap to use String as a key
	// while allowing it to use &str for gets
	pub fn load_shared<D>(&mut self, details: &D) -> Result<(Rc<R>, i64), String>
	where
		L: ResourceLoader<'l, R, Args = D>,
		D: Eq + Hash + ?Sized,
		K: Borrow<D> + for<'a> From<&'a D>,
	{
		match self.index_cache.get(details.into()) {
			None => {
				let t = self.loader.borrow();
				let resource = Rc::new(t.load(details)?);
				let index = self.cache.len();
				self.cache.push(resource.clone());
				self.index_cache.insert(details.into(), index);
				//println!("SHARED {}", index);
				Ok((self.cache.last().unwrap().clone(), index as i64))
			},
			Some(v) => Ok((self.cache[*v].clone(), *v as i64)),
		}
	}

	pub fn load_unique<D>(&mut self, details: &D) -> Result<(&R, i64), String>
	where
		L: ResourceLoader<'l, R, Args = D>,
		D: Eq + Hash + ?Sized,
		K: Borrow<D> + for<'a> From<&'a D>,
	{
		let resource = Box::new(self.loader.load(details)?);
		let index = self.unique_cache.len();
		self.unique_cache.push(Some(resource));
		//println!("UNIQUE {}", - (index as i64) - 1);
		Ok((self.unique_cache.last().as_ref().unwrap().as_ref().unwrap(), - (index as i64) - 1))
	}

	pub fn remove_unique(&mut self, index: i64) {
		self.unique_cache[i64::abs(index + 1) as usize] = None;
	}

	pub fn from_index(&self, index: i64) -> Option<&R> {
		if index < 0 {
			if let Some(resource) = self.unique_cache.get(i64::abs(index + 1) as usize) {
				if resource.is_some() {
					Some(resource.as_ref().unwrap().borrow())
				} else {
					None
				}
			} else {
				None
			}
		} else {
			if let Some(resource) = self.cache.get(index as usize) {
				Some(resource.as_ref().borrow())
			} else {
				None
			}
		}
	}

	pub fn from_index_mut(&mut self, index: i64) -> Option<&mut R> {
		if index < 0 {
			if let Some(resource) = self.unique_cache.get_mut(i64::abs(index + 1) as usize) {
				if resource.is_some() {
					Some(resource.as_mut().unwrap())
				} else {
					None
				}
			} else {
				None
			}
		} else {
			if let Some(resource) = self.cache.get_mut(index as usize) {
				Some(Rc::get_mut(resource).unwrap())
			} else {
				None
			}
		}
	}

	pub fn take_from_existing(&mut self, resource: Box<R>, existing_texture_index: Option<i64>) -> i64 {
		if let Some(id) = existing_texture_index {
			let index = if id > 0 { id } else { -id - 1 } as usize;
			self.unique_cache[index] = Some(resource);
			id
		} else {
			let id = self.unique_cache.len() + 1;
			self.unique_cache.push(Some(resource));
			-(id as i64)
		}
	}
}

// TextureCreator knows how to load Textures
impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
	type Args = str;
	fn load(&'l self, path: &str) -> Result<Texture<'l>, String> {
		println!("LOADED A TEXTURE");
		self.load_texture(path)
	}
}

// Font Context knows how to load Fonts
impl<'l> ResourceLoader<'l, Font<'l, 'static>> for Sdl2TtfContext {
	type Args = FontDetails;
	fn load(&'l self, details: &FontDetails) -> Result<Font<'l, 'static>, String> {
		println!("LOADED A FONT");
		self.load_font(&details.path, details.size)
	}
}

// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
	type Args: ?Sized;
	fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

// Information needed to load a Font
#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
	pub path: String,
	pub size: u16,
}

impl<'a> From<&'a FontDetails> for FontDetails {
	fn from(details: &'a FontDetails) -> FontDetails {
		FontDetails {
			path: details.path.clone(),
			size: details.size,
		}
	}
}

pub type TextureManager<'sdl_module, T> = ResourceManager<'sdl_module, String, Texture<'sdl_module>, TextureCreator<T>>;
pub type FontManager<'ttf_module> = ResourceManager<'ttf_module, FontDetails, Font<'ttf_module, 'static>, Sdl2TtfContext>;
