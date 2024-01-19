use bevy::ecs::event::Event;
use bevy::ecs::entity::Entity;

#[derive(Clone, Default, Event)]
pub struct ShakeEvent {
	pub intensity: f32,
}

#[derive(Clone, Event)]
pub struct IntersectEvent {
	pub ab: (Entity, Entity),
}
