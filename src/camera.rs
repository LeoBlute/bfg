use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use rand::prelude::*;

use crate::components;
use crate::events;
use crate::system;

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup)
		.add_systems(
			Update,
			(apply_camera_shake, camera_follow_player, camera_shake_event_read).in_set(system::GameSet::Apply)
		);
	}
}

fn setup(mut commands: Commands) {
	//Spawns camera with a different scaling mode than the default Camera2dBundle
    commands.spawn((SpatialBundle::default(), components::CameraFollow(8.0)))
	.with_children(|parent| {
		parent.spawn((
			Camera2dBundle {
				projection: OrthographicProjection {
					far: 1000.0,
					near: -1000.0,
					scale: 2.0,
					scaling_mode: ScalingMode::AutoMin {
						min_width: 256.0,
						min_height: 144.0,
					},
					..default()
				},
				..default()
			},
			components::CameraShake {
				intensity: 0.0,
				recover  : 9.0,
				x:         1.0,
				y:         1.0,
				z:         1.0,
				..default()
			},
		));
	});
}

fn apply_camera_shake(
	mut query: Query<(&mut components::CameraShake, &mut Transform)>, 
	r_time: Res<Time>,
) {
	let Ok((mut values, mut transform)) = query.get_single_mut() else { return };

	let dt = r_time.delta_seconds();
	let intensity = values.intensity * 0.05;
	let mut rng = rand::thread_rng();

	let rot_z = intensity * values.z * rng.gen_range(-1.0..=1.0);
	let intensity = intensity * 10.0;

	let offset_x = intensity * values.x * rng.gen_range(-1.0..=1.0);
	let offset_y = intensity * values.y * rng.gen_range(-1.0..=1.0);

	transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rot_z);

	transform.translation = Vec3::new(offset_x, offset_y, 0.0);

	values.intensity -= dt * values.recover;
	values.intensity = values.intensity.clamp(0.0, 100.0);
}

fn camera_shake_event_read(
	mut er_shake: EventReader<events::ShakeEvent>,
	mut query: Query<&mut components::CameraShake>,
) {
	let Ok(mut camera_shake) = query.get_single_mut() else { return };
	for event in er_shake.read() {
		camera_shake.intensity += event.intensity;
	}
}

fn camera_follow_player(
	q_player: Query<&Transform, With<components::PlayerCharacter>>,
	mut q_camera: Query<(&mut Transform, &components::CameraFollow), Without<components::PlayerCharacter>>,
	r_time: Res<Time>,
) {
	let Ok((mut camera_transform, speed)) = q_camera.get_single_mut() else { return };
	let Ok(player_transform) = q_player.get_single() else { return };

	let dest = Vec3::new(player_transform.translation.x, player_transform.translation.y, 0.0);
	camera_transform.translation = camera_transform.translation.lerp(dest, speed.0 * r_time.delta_seconds());
}
