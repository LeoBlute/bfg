use bevy::prelude::*;

#[derive(Clone, Default)]
pub struct SystemSchedulePlugin;

impl Plugin for SystemSchedulePlugin {
	fn build(&self, app: &mut App) {
		app.configure_sets(Update, (
			GameSet::Check,
			GameSet::CommandsAction,
			GameSet::SingleAction,
			GameSet::ContinousAction,
			GameSet::Apply,
			GameSet::Despawn,
		).chain())
		.add_systems(Update, apply_deferred.after(GameSet::CommandsAction).before(GameSet::SingleAction));
	}
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSet {
	Check,
	CommandsAction,
	SingleAction,
	ContinousAction,
	Apply,
	Despawn,
}
