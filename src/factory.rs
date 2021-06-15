use rand::Rng;

use crate::{components::{ai::{DestinationPoint, TrajectorySequence}, force::ForceComponent, health::HealthComponent, hitbox::HitboxComponent, input::InputComponent, lifetime::LifetimeComponent, shot::{ShotComponent, ShotType}, sprite::{SpriteComponent, Spritesheet, SpritesheetOrientation}, transform::TransformComponent}, core::{common::{self, GameServices}, ecs::EntityId}};

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
	let force = world.get_component_mut::<ForceComponent>(&entity).unwrap();
	force.vx = vx;
	force.vy = vy;
	let hitbox = &mut game_services.get_world_mut().get_component_mut::<HitboxComponent>(&entity).unwrap().hitbox;
	hitbox.w /= 2;
	hitbox.h /= 2;
	hitbox.x += hitbox.w / 2;
	hitbox.y += hitbox.h / 2;
	entity
}

fn generate_circle_point(origin: &DestinationPoint, radius: &u32, angle: &f32) -> DestinationPoint {
	(origin.0 + *radius as f32 * f32::cos(*angle), origin.1 + *radius as f32 * f32::sin(*angle))
}

fn generate_circle_pattern(origin: &DestinationPoint, circle_radius: u32, angle_start_degrees: i32, angle_end_degrees: i32, step_precision_portions: usize) -> TrajectorySequence {
	assert!(angle_start_degrees < angle_end_degrees);
	let mut sequence = TrajectorySequence::new();
	let step = ((angle_end_degrees - angle_start_degrees) / step_precision_portions as i32) as usize;
	for angle in (angle_start_degrees..(angle_end_degrees + step as i32)).step_by(step) {
		let radian_angle = - angle as f32 * std::f32::consts::PI / 180.0;
		sequence.push(generate_circle_point(origin, &circle_radius, &radian_angle));
	}
	sequence
}

fn generate_line_pattern(origin: &DestinationPoint, destination: &DestinationPoint) -> TrajectorySequence {
	let mut sequence = TrajectorySequence::new();
	sequence.push(*origin);
	sequence.push(*destination);
	sequence.shoot_delay_ms = 4000;
	sequence
}

pub fn generate_enemy_movement_pattern(start_time_ms: u64, screen_center: DestinationPoint) -> Vec<TrajectorySequence> {
	let mut sequence = Vec::new();
	let start_pos = (screen_center.0, screen_center.1 / 2.0);

	sequence.push(TrajectorySequence::wait(start_time_ms));

	let mut circle = generate_circle_pattern(&start_pos, 100, -90, 90, 10);
	circle.shoot_delay_ms = 3000;
	let last_circle_point = circle.last().unwrap().clone();
	sequence.push(circle);
	let final_pos = (screen_center.0, screen_center.1 * 2.5);
	sequence.push(generate_line_pattern(&last_circle_point, &final_pos));

	sequence
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
		sprite_component.spritesheet = Some(Spritesheet::new(num, orientation, src_width, src_height));
		sprite_component.animation_delay = delay;
		game_services.get_world_mut().add_component(&entity, sprite_component);
	}
	game_services.get_world_mut().add_component(&entity, TransformComponent::new(x as f32, y as f32));
	entity
}
