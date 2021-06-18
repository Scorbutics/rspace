use rand::{Rng};

use crate::{components::{ai::{DestinationPoint}, animation::AnimationComponent, force::ForceComponent, health::HealthComponent, hitbox::HitboxComponent, input::InputComponent, lifetime::LifetimeComponent, shot::{ShotComponent, ShotType}, sprite::{SpriteComponent, Spritesheet, SpritesheetOrientation}, transform::TransformComponent}, core::{animation::Animation, common::{self, GameServices}, ecs::EntityId}};

pub fn create_entity<'sdl_all, 'world>(texture_name: &str, x: i32, y: i32, width: u32, height: u32, game_services: &mut GameServices<'sdl_all, 'world>) -> EntityId {
	let entity = game_services.get_world_mut().create_entity();
	if ! texture_name.is_empty() {
		let sprite = game_services.resource_manager.load_texture(&texture_name);
		game_services.get_world_mut().add_component(&entity, SpriteComponent::new(sprite.unwrap(), width, height));
	}
	game_services.get_world_mut().add_component(&entity, HitboxComponent::new(0, 0, width, height));
	game_services.get_world_mut().add_component(&entity, TransformComponent::new(x as f32, y as f32));
	entity
}

pub fn create_physics_entity<'sdl_all, 'world>(texture_name: &str, x: i32, y: i32, width: u32, height: u32, game_services: &mut GameServices<'sdl_all, 'world>) -> EntityId  {
	let entity = create_entity(texture_name, x, y, width, height, game_services);
	game_services.get_world_mut().add_component(&entity, ForceComponent::new());
	entity
}

pub fn create_living_entity<'sdl_all, 'world>(texture_name: &str, x: i32, y: i32, width: u32, height: u32, game_services: &mut GameServices<'sdl_all, 'world>) -> EntityId  {
	let entity = create_physics_entity(texture_name, x, y, width, height, game_services);
	game_services.get_world_mut().add_component(&entity, HealthComponent::new(1));

	let sprite_component = game_services.get_world_mut().get_component_mut::<SpriteComponent>(&entity).unwrap();
	sprite_component.spritesheet = Some(Spritesheet::new(3, 2, SpritesheetOrientation::HORIZONTAL, 16, 16));

	let mut animation_component = AnimationComponent::new();
	let animation = Animation::new(0).frames(3).time(80).count(1).clone();

	let stand_to_right = animation.clone().origin(1).clone();
	let right_to_stand = stand_to_right.clone().reverse().offset(2).clone();
	let stand_to_left = animation.clone();
	let left_to_stand = stand_to_left.clone().reverse().offset(2).clone();

	let right_to_left = right_to_stand.clone().then(&stand_to_left).clone();
	let left_to_right = left_to_stand.clone().then(&stand_to_right).clone();

	animation_component.set(vec![stand_to_right, right_to_stand, stand_to_left, left_to_stand, right_to_left, left_to_right]);
	game_services.get_world_mut().add_component(&entity, animation_component);
	entity
}

pub fn create_player<'sdl_all, 'world>(texture_name: &str, x: i32, y: i32, width: u32, height: u32, power: f32, game_services: &mut GameServices<'sdl_all, 'world>) -> EntityId {
	let entity = create_living_entity(texture_name, x, y, width, height, game_services);
	game_services.get_world_mut().add_component(&entity, InputComponent::new(power, true));
	entity
}

pub fn create_shot<'sdl_all, 'world>(texture_name: &str, x: i32, y: i32, width: u32, height: u32, vx: f32, vy: f32, lifetime: u64, origin: ShotType, game_services: &mut GameServices<'sdl_all, 'world>) -> EntityId {
	let entity = create_physics_entity(texture_name, x, y, width, height, game_services);
	let world = game_services.get_world_mut();
	world.add_component(&entity, ShotComponent::new(origin, 1));
	world.add_component(&entity, LifetimeComponent::new(common::current_time_ms() + lifetime));

	let sprite_component = world.get_component_mut::<SpriteComponent>(&entity).unwrap();
	sprite_component.spritesheet = Some(Spritesheet::new(7, 1, SpritesheetOrientation::HORIZONTAL, 16, 16));
	let mut animation_component = AnimationComponent::new();
	let animation = Animation::new(0).frames(7).time(40).count(1).start().clone();
	animation_component.set(vec![animation]);
	animation_component.next(0);
	world.add_component(&entity, animation_component);

	let force = world.get_component_mut::<ForceComponent>(&entity).unwrap();
	force.vx = vx;
	force.vy = vy;
	let hitbox = &mut world.get_component_mut::<HitboxComponent>(&entity).unwrap().hitbox;
	hitbox.w /= 2;
	hitbox.h /= 2;
	hitbox.x += hitbox.w / 2;
	hitbox.y += hitbox.h / 2;
	entity
}

pub fn random_outside_spawn_pos(screen_width: u32, screen_height: u32) -> DestinationPoint {
	let mut rng = rand::thread_rng();
	let random_side= rng.gen_range(1, 4) as i16;
	match random_side {
		1 => {
			// Left
			(0.0, rng.gen_range(0.0, screen_height as f32 / 2.0) as f32)
		},
		2 => {
			// Right
			(screen_width as f32, rng.gen_range(0.0, screen_height as f32 / 2.0) as f32)
		},
		_ => {
			// Up
			(rng.gen_range(0.0, screen_width as f32) as f32, 0.0)
		}
	}
}

pub fn create_animation<'sdl_all, 'world>(texture_name: &str, x: i32, y: i32, src_width: u32, src_height: u32, dst_width: u32, dst_height: u32, num: usize, orientation: SpritesheetOrientation, delay: u64, game_services: &mut GameServices<'sdl_all, 'world>) -> EntityId {
	let entity = game_services.get_world_mut().create_entity();
	if ! texture_name.is_empty() {
		let sprite = game_services.resource_manager.load_texture(&texture_name);
		let mut sprite_component = SpriteComponent::new(sprite.unwrap(), dst_width, dst_height);
		sprite_component.spritesheet = Some(Spritesheet::new(num, 1, orientation, src_width, src_height));
		game_services.get_world_mut().add_component(&entity, sprite_component);
		let mut animation_component = AnimationComponent::new();
		let animation = Animation::new(0).frames(8).time(delay).count(1).start().clone();
		animation_component.set(vec![animation]);
		game_services.get_world_mut().add_component(&entity, animation_component);
	}
	game_services.get_world_mut().add_component(&entity, TransformComponent::new(x as f32, y as f32));
	entity
}
