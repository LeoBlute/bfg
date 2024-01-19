use bevy::prelude::*;

#[derive(Clone, Default, Component)]
pub struct WalkAnimate {
	pub spd           : f32,
	pub intensity     : f32,
	pub step_intensity: f32,
	pub ghosting_timer: Option<Timer>
}

#[derive(Clone, Default, Component)]
pub struct PlayerWeapon {
	pub damage: f32,
}

#[derive(PartialEq, Clone, Default, Component)]
pub struct Intersect {
	pub size: Vec2,
}

#[derive(Clone, Default, Component, Debug)]
pub struct Health {
	pub current         : f32,
	pub unapplied_damage: f32,
}

#[derive(Clone, Default, Component)]
pub struct DespawnerTimer {
	pub timer: Timer,
}

#[derive(Clone, Default, Component)]
pub struct Velocity {
	pub v: Vec2,
}

#[derive(Clone, Default, Component)]
pub struct PlayerCharacter;

#[derive(Clone, Component)]
pub struct HealthMeter {
	pub id: Entity,
}

#[derive(Clone, Default, Component)]
pub struct CameraFollow(pub f32);

#[derive(Clone, Default, Component)]
pub struct CameraShake {
	pub intensity: f32,
	pub recover  : f32,
	pub x        : f32,
	pub y        : f32,
	pub z        : f32,
}

#[derive(Clone, Default, Component)]
pub struct DamageZone {
	pub damage: f32,
	pub ignore: Option<Entity>,
}

#[derive(Clone, Default, Component)]
pub struct Boss {
	pub speed: f32,
}

impl DespawnerTimer {
	pub fn new(despawn_time: bevy::utils::Duration) -> Self {
		Self { timer: Timer::new(despawn_time, TimerMode::Once) }
	}
}

impl WalkAnimate {
	pub fn new(speed: f32, intensity: f32, step_intensity: f32) -> Self {
		Self { 
			spd: speed,
			intensity: intensity,
			step_intensity: step_intensity,
			ghosting_timer: None 
		}
	}

	pub fn new_with_marker(speed: f32, intensity: f32, step_intensity: f32, tracking_time: bevy::utils::Duration) -> Self {
		Self {
			spd: speed,
			intensity: intensity,
			step_intensity: step_intensity,
			ghosting_timer: Some(Timer::new(tracking_time, TimerMode::Repeating))
		}
	}
}
