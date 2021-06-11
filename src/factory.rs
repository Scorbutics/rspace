use crate::{components::{ai::TimeFunction, force::ForceComponent, hitbox::HitboxComponent, input::InputComponent, lifetime::LifetimeComponent, shot::{ShotComponent, ShotType}, sprite::SpriteComponent, transform::TransformComponent}, core::{common::{self, GameServices}, ecs::EntityId}};

pub fn create_entity<'sdl_all, 'world>(texture_name: &str, x: i32, y: i32, width: u32, height: u32, game_services: &mut GameServices<'sdl_all, 'world>) -> EntityId {
	let entity = game_services.get_world_mut().create_entity();
	if ! texture_name.is_empty() {
		let sprite = game_services.resource_manager.load_texture(&texture_name);
		game_services.get_world_mut().add_component(&entity, SpriteComponent::new(sprite.unwrap()));
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

pub fn create_player<'sdl_all, 'world>(texture_name: &str, x: i32, y: i32, width: u32, height: u32, power: f32, game_services: &mut GameServices<'sdl_all, 'world>) -> EntityId {
	let entity = create_physics_entity(texture_name, x, y, width, height, game_services);
	game_services.get_world_mut().add_component(&entity, InputComponent::new(power, true));
	entity
}

pub fn create_shot<'sdl_all, 'world>(texture_name: &str, x: i32, y: i32, width: u32, height: u32, vx: f32, vy: f32, lifetime: u64, game_services: &mut GameServices<'sdl_all, 'world>) -> EntityId {
	let entity = create_physics_entity(texture_name, x, y, width, height, game_services);
	let world = game_services.get_world_mut();
	world.add_component(&entity, ShotComponent::new(ShotType::PLAYER));
	world.add_component(&entity, LifetimeComponent::new(common::current_time_ms() + lifetime));
	let force = world.get_component_mut::<ForceComponent>(&entity).unwrap();
	force.vx = vx;
	force.vy = vy;
	entity
}

fn circle_pattern(time: u64, power: f32, pos: (f32, f32)) -> (f32, f32) {
	let dir = f32::atan2(pos.1, pos.0);
	(f32::cos(dir), f32::sin(dir))
}

pub fn generate_enemy_movement_pattern() -> Box<TimeFunction> {
	Box::new(circle_pattern)
}
