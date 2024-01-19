use bevy::{
	prelude::*, input::common_conditions::*
};

use crate::components;
use crate::system;
use crate::events;

use crate::miscellaneous;

const WEAPON_DAMAGE: f32 = 5.0;

#[derive(Default, Clone)]
pub struct PlayerWeaponPlugin;

impl Plugin for PlayerWeaponPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_systems(
			Update,
			player_weapon_attack.in_set(system::GameSet::CommandsAction).run_if(input_just_pressed(MouseButton::Left))
		)
		.add_systems(Update, player_weapon_transformation.in_set(system::GameSet::Apply))
		.add_systems(Startup, setup);
	}
}

fn setup(
	mut commands: Commands,
	r_asset_server: Res<AssetServer>
) {
	let weapon_texture = r_asset_server.load("textures/rpg/props/generic-rpg-loot01.png");
	commands.spawn((
		components::PlayerWeapon { damage: WEAPON_DAMAGE },
		components::Intersect { size: Vec2::splat(12.0) },
		SpriteBundle {
			sprite: Sprite {
				color: Color::rgba(1.0, 1.0, 1.0, 0.6),
				custom_size: Some(Vec2::splat(12.0)),
				..default()
			},
			transform: Transform::from_xyz(0.0, 0.0, 900.0),
			texture: weapon_texture,
			..default()
		}
	));
}

fn player_weapon_attack(
	mut commands: Commands,
	q_weapon: Query<(Entity, &Transform, &components::PlayerWeapon)>,
	mut q_health: Query<&mut components::Health, Without<components::PlayerCharacter>>,
	mut er_intersection: EventReader<events::IntersectEvent>,
	mut ew_shake: EventWriter<events::ShakeEvent>,
) {
	let Ok((id, transform, weapon)) = q_weapon.get_single() else {
		return
	};

	miscellaneous::generic_particle_burst(
		&mut commands,
		transform.translation,
		100.0,
		0.5,
		1.0,
		50,
		(Color::WHITE..Color::rgba(1.0, 1.0, 1.0, 0.0)).into()
	);

	ew_shake.send(events::ShakeEvent { intensity: 1.0 });

	for ev in er_intersection.read() {
		let (a, b) = ev.ab;

		if id != a {
			continue;
		};

		let Ok(mut health) = q_health.get_mut(b) else {
			continue;
		};

		//Once all the checks are done the damage is applied
		health.unapplied_damage += weapon.damage;
	}
}

fn player_weapon_transformation(
	mut q_weapon: Query<&mut Transform, With<components::PlayerWeapon>>,
	q_player:     Query<&Transform, (With<components::PlayerCharacter>, Without<components::PlayerWeapon>)>,
	q_window: Query<&Window, With<bevy::window::PrimaryWindow>>,
	q_camera: Query<(&Camera, &GlobalTransform)>,
) {
	let(
		Ok(mut weapon_transform), 
		Ok(player_transform),
		Ok(window),
		Ok((camera, gt_camera))
		) = (
			q_weapon.get_single_mut(),
			q_player.get_single(),
			q_window.get_single(),
			q_camera.get_single()
	)
	else {
		return
	};

	let Some(cursor_pos) = window.cursor_position()
		.and_then(|cursor| camera.viewport_to_world_2d(gt_camera, cursor))
	else { return };

	let mut translation = player_transform.translation;
	translation.z = 950.0;
	let v = cursor_pos - translation.truncate();
	let b = v.normalize();
	let rot = Quat::from_rotation_arc_2d(Vec2::Y, b).normalize();
	let direction = rot * Vec3::Y;
	let translation = translation + (direction * 30.0);

	weapon_transform.translation = translation;
}
