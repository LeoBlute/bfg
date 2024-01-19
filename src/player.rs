use bevy::{
	prelude::*, input::common_conditions::*
};

use crate::system;
use crate::components;

#[derive(Clone, Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, player_char_controls.in_set(system::GameSet::ContinousAction))
		.add_systems(Update, player_dash_ability.run_if(input_just_pressed(KeyCode::F)).in_set(system::GameSet::SingleAction));
	}
}

fn player_dash_ability(
	r_inputs: Res<Input<KeyCode>>,
	mut query: Query<&mut components::Velocity, With<components::PlayerCharacter>>,
) {
	//this check is made with run critirea
	//if !inputs.just_pressed(KeyCode::F) {
	//	return
	//}

	let Ok(mut velocity) = query.get_single_mut() else { return };
	let mut impulse = Vec2::ZERO;
	let impulse_amount = 10.0 * 1000.0;

	if r_inputs.pressed(KeyCode::A) {
		impulse.x -= impulse_amount;
	}

	if r_inputs.pressed(KeyCode::D) {
		impulse.x += impulse_amount;
	}

	if r_inputs.pressed(KeyCode::W) {
		impulse.y += impulse_amount;
	}

	if r_inputs.pressed(KeyCode::S) {
		impulse.y -= impulse_amount;
	}
		
	velocity.v += impulse;
}

fn player_char_controls(
	r_inputs: Res<Input<KeyCode>>,
	mut query: Query<&mut components::Velocity, With<components::PlayerCharacter>>,
) {
	let Ok(mut velocity) = query.get_single_mut() else { return };
	let mut impulse = Vec2::ZERO; 
	let speed = 120.0;

	if r_inputs.pressed(KeyCode::A) {
		impulse.x -= speed;
	}

	if r_inputs.pressed(KeyCode::D) {
		impulse.x += speed;
	}

	if r_inputs.pressed(KeyCode::W) {
		impulse.y += speed;
	}

	if r_inputs.pressed(KeyCode::S) {
		impulse.y -= speed;
	}
	velocity.v += impulse;
}
