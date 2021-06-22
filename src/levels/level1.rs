use super::{phase_basic_spawn::{LevelPhaseBasicSpawn, SpawnerPositionProperty}, phases::{TrajectoryType}};

pub struct Level1Start {
}


impl Level1Start {
	pub fn new() -> LevelPhaseBasicSpawn {
		let mut base = LevelPhaseBasicSpawn::new(30.0);
		let last = base.add(TrajectoryType::BasicDiagonalLeft, 5, 3000, 3, 1.0);
		last.position_prop = SpawnerPositionProperty::Fixed;
		//last.luck = 50.0;
		let last = base.add(TrajectoryType::ReverseDiagonalRight, 5, 3000, 3, 1.0);
		last.position_prop = SpawnerPositionProperty::Fixed;
		//last.luck = 50.0;
		base
	}
}
pub struct Level1Mid {
}

impl Level1Mid {
	pub fn new() -> LevelPhaseBasicSpawn {
		let mut base = LevelPhaseBasicSpawn::new(45.0);
		let last = base.add(TrajectoryType::BasicLinear, 4, 2500, 3, 1.5);
		last.position_prop = SpawnerPositionProperty::AlternateSymetric;
		base
	}
}

pub struct Level1Mid2 {
}

impl Level1Mid2 {
	pub fn new() -> LevelPhaseBasicSpawn {
		let mut base = LevelPhaseBasicSpawn::new(60.0);
		base.add(TrajectoryType::BasicCircle, 5, 3000, 3, 1.5);
		base
	}
}

pub struct Level1End {
}

impl Level1End {
	pub fn new() -> LevelPhaseBasicSpawn {
		let mut base = LevelPhaseBasicSpawn::new(20.0);
		let last = base.add(TrajectoryType::CenteredCircle, 8, 2500, 3, 1.0);
		last.position_prop = SpawnerPositionProperty::Fixed;
		base.add(TrajectoryType::BasicDiagonalLeft, 2, 5000, 3, 1.5);
		base.add(TrajectoryType::BasicDiagonalRight, 2, 5000, 3, 1.5);
		base
	}
}
