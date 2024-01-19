use bevy::prelude::*;

use crate::components;
use crate::system;

const BOSS_SPEED: f32 = 60.0;

#[derive(Clone, Default)]
pub struct BossPlugin;

impl Plugin for BossPlugin {
	fn build(&self, app: &mut App) {
		app
		.add_systems(Update, boss_movement.in_set(system::GameSet::ContinousAction))
		.add_systems(Startup, setup);
	}
}

fn boss_movement(
	mut q_velocity: Query<(&mut components::Velocity, &Transform, &components::Boss)>,
	q_player_transform: Query<&Transform, (With<components::PlayerCharacter>, Without<components::Boss>)>
) {
	let Ok((mut velocity, transform, values)) = q_velocity.get_single_mut() else { return };
	let Ok(target_transform) = q_player_transform.get_single() else { return };

	let pos = transform.translation.truncate();
	let target_pos = target_transform.translation.truncate();

	let direction = -(pos - target_pos).normalize();
	velocity.v += direction * values.speed;
}

fn setup(
	mut commands: Commands,
	r_asset_server: Res<AssetServer>,
    mut rm_texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let enemy_texture = r_asset_server.load("textures/rpg/chars/hat-guy/hat-guy.png");
	let size = Vec2::new(16.0, 22.0);
    let enemy_texture_atlas =
        TextureAtlas::from_grid(enemy_texture, size, 1, 1, None, None);
    let enemy_texture_atlas = rm_texture_atlases.add(enemy_texture_atlas);

	let size = size * 2.0;
	let enemy_id = commands.spawn((
		SpatialBundle {
			transform: Transform::from_xyz(0.0, 200.0, 900.0),
			..default()
		},
		components::Velocity::default(),
		components::Health { current: 100.0, ..default() },
		components::Intersect { size },
		components::DamageZone { damage: 2.0, ignore: None },
		components::Boss {
			speed: BOSS_SPEED,
			..default()
		},
		components::WalkAnimate::new(20.0, 1.0, 2.0)
	)).with_children(|parent| {
		parent.spawn(
			SpriteSheetBundle {
				sprite: TextureAtlasSprite {
					custom_size: Some(size),
					index: 0,
					..default()
				},
            	texture_atlas: enemy_texture_atlas,
            	..default()
			},
		);
	}).id();

	commands.spawn((
		NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				right: Val::Px(0.0),
				width: Val::Px(25.0),
				height: Val::Px(100.0),
				..default()
			},
			background_color: Color::RED.into(),
			..default()
		},
		components::HealthMeter { id: enemy_id },
	));
}
