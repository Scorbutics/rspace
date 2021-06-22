use crate::{components::ai::{DestinationPoint, TrajectorySequence}, core::common::GameServices};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TrajectoryType {
	BasicCircle,
	BasicLinear,
	BasicDiagonalLeft,
	BasicDiagonalRight,
	ReverseDiagonalLeft,
	ReverseDiagonalRight,
	CenteredCircle
}

pub trait LevelPhase {
	fn on_enter<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>);
	fn update<'sdl_all, 'l>(&mut self, game_services: &mut GameServices<'sdl_all,'l>) -> bool;
}

pub struct TrajectoryGenerator {
}

impl TrajectoryGenerator {
	fn generate_circle_point(origin: &DestinationPoint, radius: &u32, angle: &f32) -> DestinationPoint {
		(origin.0 + *radius as f32 * f32::cos(*angle  % (2.0 * std::f32::consts::PI)), origin.1 + *radius as f32 * f32::sin(*angle % (2.0 * std::f32::consts::PI)))
	}

	fn generate_circle_pattern(origin: &DestinationPoint, circle_radius: u32, angle_start_degrees: i32, angle_end_degrees: i32, step_precision_portions: usize) -> TrajectorySequence {
		assert!(angle_start_degrees < angle_end_degrees);
		let mut sequence = TrajectorySequence::new();
		let step = ((angle_end_degrees - angle_start_degrees) / step_precision_portions as i32) as usize;
		for angle in (angle_start_degrees..(angle_end_degrees + step as i32)).step_by(step) {
			let radian_angle = - angle as f32 * std::f32::consts::PI / 180.0;
			let point = Self::generate_circle_point(origin, &circle_radius, &radian_angle);
			sequence.push(point);
		}
		sequence
	}

	fn generate_line_pattern(origin: &DestinationPoint, destination: &DestinationPoint) -> TrajectorySequence {
		let mut sequence = TrajectorySequence::new();
		sequence.push(*origin);
		sequence.push(*destination);
		sequence
	}

	fn enqueue_pattern_basic_circle(sequence: &mut Vec<TrajectorySequence>, start_pos: DestinationPoint) {
		let mut line_start = TrajectorySequence::new();
		line_start.push((start_pos.0, start_pos.1 + 100.0));
		line_start.set_shoot_delay(10000);
		line_start.shoot_num = 1;
		sequence.push(line_start);

		let mut circle = Self::generate_circle_pattern(&start_pos, 100, -90, 90, 10);
		circle.set_shoot_delay(1000);
		circle.shoot_num = 3;
		let last_circle_point = circle.last().unwrap().clone();
		sequence.push(circle);
		let final_pos = (start_pos.0, start_pos.1 * 2.5);

		let mut line_end = Self::generate_line_pattern(&last_circle_point, &final_pos);
		line_end.set_shoot_delay(10000);
		line_end.shoot_num = 1;
		sequence.push(line_end);
	}

	fn enqueue_pattern_centered_circle(sequence: &mut Vec<TrajectorySequence>, start_pos: DestinationPoint) {
		let mut circle = Self::generate_circle_pattern(&start_pos, 180, 90, 432, 20);
		circle.set_shoot_delay(2000);
		circle.shoot_num = 1;
		circle.loop_count = u16::MAX;
		sequence.push(circle);
	}

	fn enqueue_pattern_basic_linear(sequence: &mut Vec<TrajectorySequence>, start_y: f32, screen_width: u32) {
		let pos_left = (90.0, start_y);
		let pos_right = (screen_width as f32 - 90.0, start_y);

		let mut line_start = TrajectorySequence::new();
		line_start.push(pos_left);
		line_start.set_shoot_delay(10000);
		line_start.shoot_num = 2;
		sequence.push(line_start);

		for _i in 0..2 {
			let mut right_move = Self::generate_line_pattern(&pos_left, &pos_right);
			right_move.set_shoot_delay(3000);
			right_move.shoot_num = 1;
			sequence.push(right_move);
			let mut left_move = Self::generate_line_pattern(&pos_right, &pos_left);
			left_move.set_shoot_delay(3000);
			left_move.shoot_num = 1;
			sequence.push(left_move);
		}

		let final_pos = (- 90.0, start_y);
		let mut line_end = Self::generate_line_pattern(&pos_left, &final_pos);
		line_end.set_shoot_delay(10000);
		line_end.shoot_num = 2;
		sequence.push(line_end);
	}

	fn enqueue_pattern_basic_diagonal_left(sequence: &mut Vec<TrajectorySequence>, screen_width: u32, screen_height: u32) {
		let pos_left = (90.0, screen_height as f32 - 200.0);
		let pos_right = (screen_width as f32 - 90.0, 90.0);

		let mut diagonal = Self::generate_line_pattern(&pos_left, &pos_right);
		diagonal.set_shoot_delay(2000);
		diagonal.shoot_num = 2;
		sequence.push(diagonal);

		let mut ffinal = TrajectorySequence::new();
		ffinal.push((screen_width as f32 + 90.0, - 90.0));
		sequence.push(ffinal);
	}

	fn enqueue_pattern_reverse_diagonal_left(sequence: &mut Vec<TrajectorySequence>, screen_width: u32, screen_height: u32) {
		let pos_left = (90.0, screen_height as f32 - 200.0);
		let pos_right = (screen_width as f32 - 90.0, 90.0);

		let mut diagonal = Self::generate_line_pattern(&pos_right, &pos_left);
		diagonal.set_shoot_delay(2000);
		diagonal.shoot_num = 2;
		sequence.push(diagonal);

		let mut ffinal = TrajectorySequence::new();
		ffinal.push((screen_width as f32 + 90.0, - 90.0));
		sequence.push(ffinal);
	}

	fn enqueue_pattern_basic_diagonal_right(sequence: &mut Vec<TrajectorySequence>, screen_width: u32, screen_height: u32) {
		let pos_left = (90.0, 90.0);
		let pos_right = (screen_width as f32 - 90.0, screen_height as f32 - 200.0);

		let mut diagonal = Self::generate_line_pattern(&pos_left, &pos_right);
		diagonal.set_shoot_delay(2000);
		diagonal.shoot_num = 2;
		sequence.push(diagonal);

		let mut ffinal = TrajectorySequence::new();
		ffinal.push((screen_width as f32 + 90.0, screen_height as f32 + 90.0));
		sequence.push(ffinal);
	}

	fn enqueue_pattern_reverse_diagonal_right(sequence: &mut Vec<TrajectorySequence>, screen_width: u32, screen_height: u32) {
		let pos_left = (90.0, 90.0);
		let pos_right = (screen_width as f32 - 90.0, screen_height as f32 - 200.0);

		let mut diagonal = Self::generate_line_pattern(&pos_right, &pos_left);
		diagonal.set_shoot_delay(2000);
		diagonal.shoot_num = 2;
		sequence.push(diagonal);

		let mut ffinal = TrajectorySequence::new();
		ffinal.push((screen_width as f32 + 90.0, screen_height as f32 + 90.0));
		sequence.push(ffinal);
	}

	pub fn generate_enemy_movement_pattern(trajectory: &TrajectoryType, start_time_ms: u64, screen_center: DestinationPoint, screen_width: u32, screen_height: u32) -> Vec<TrajectorySequence> {
		let mut sequence = Vec::new();
		let start_pos = (screen_center.0, screen_center.1 / 2.0);
		sequence.push(TrajectorySequence::wait(start_time_ms));

		match *trajectory {
			TrajectoryType::BasicCircle => Self::enqueue_pattern_basic_circle(&mut sequence, start_pos),
			TrajectoryType::BasicLinear => Self::enqueue_pattern_basic_linear(&mut sequence, start_pos.1, screen_width),
			TrajectoryType::BasicDiagonalLeft => Self::enqueue_pattern_basic_diagonal_left(&mut sequence, screen_width, screen_height),
			TrajectoryType::BasicDiagonalRight => Self::enqueue_pattern_basic_diagonal_right(&mut sequence, screen_width, screen_height),
			TrajectoryType::CenteredCircle => Self::enqueue_pattern_centered_circle(&mut sequence, start_pos),
			TrajectoryType::ReverseDiagonalLeft => Self::enqueue_pattern_reverse_diagonal_left(&mut sequence, screen_width, screen_height),
			TrajectoryType::ReverseDiagonalRight => Self::enqueue_pattern_reverse_diagonal_right(&mut sequence, screen_width, screen_height),
		}
		sequence
	}
}
