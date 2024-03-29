use bevy::prelude::*;

use bevy_particle_systems::*;

use bevy::utils::Duration as BevyDuration;

mod events;
mod system;
mod components;
mod bundles;

mod miscellaneous;
mod player_weapon;
mod player;
mod camera;
mod map;
mod boss;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
		.add_plugins(ParticleSystemPlugin)
		.add_plugins((
			miscellaneous::MiscellaneousPlugin,
			camera::PlayerCameraPlugin,
			map::MapPlugin,
			player::PlayerPlugin,
			player_weapon::PlayerWeaponPlugin,
			system::SystemSchedulePlugin,
			boss::BossPlugin
		))
		.add_event::<events::ShakeEvent>()
		.add_event::<events::IntersectEvent>()
		.add_systems(Startup, setup)
		.add_systems(Update, boss_death.in_set(system::GameSet::CommandsAction))
		.add_systems(Update, player_death.in_set(system::GameSet::CommandsAction))
		.add_systems(Update, bevy::window::close_on_esc)
		.run();
}

fn boss_death(
	mut commands: Commands,
	q_boss: Query<(Entity, &components::Health), With<components::Boss>>
) {
	let Ok((id, health)) = q_boss.get_single() else { return };

	if health.current > 0.0 { return };
	commands.entity(id).despawn_recursive();
	commands.spawn(NodeBundle { 
        style: Style { 
            flex_direction: FlexDirection::Column, 
            width: Val::Percent(100.0), 
            ..default()
		}, 
        ..default() 
    })
    .with_children(|commands| {
		commands.spawn(NodeBundle { 
			style: Style {
				flex_direction: FlexDirection::Row, 
				justify_content:JustifyContent::Center, 
				..default() 
			}, 
			..default() 
		})
		.with_children(|commands| {
			let text_bundle = TextBundle::from_section(
				format!("You've killed the boss, there is nothing to be done beyound here"),
				TextStyle {
					font_size: 60.0,
					color: Color::RED,
					..default()
				},
			)
			.with_text_alignment(TextAlignment::Center);
			commands.spawn(text_bundle);
		});
    });
}

fn player_death(
	mut commands: Commands,
	q_player: Query<(Entity, &components::Health), With<components::PlayerCharacter>>,
	q_player_weapon: Query<Entity, With<components::PlayerWeapon>>,
) {
	let Ok((id, health)) = q_player.get_single() else { return };

	if health.current > 0.0 { return };

	commands.entity(id).despawn_recursive();
	commands.spawn(NodeBundle { 
        style: Style { 
            flex_direction: FlexDirection::Column, 
            width: Val::Percent(100.0), 
            ..default()
		}, 
        ..default() 
    })
    .with_children(|commands| {
		commands.spawn(NodeBundle { 
			style: Style {
				flex_direction: FlexDirection::Row, 
				justify_content:JustifyContent::Center, 
				..default() 
			}, 
			..default() 
		})
		.with_children(|commands| {
			let text_bundle = TextBundle::from_section(
				format!("You've died, there is nothing to be done anymore"),
				TextStyle {
					font_size: 60.0,
					color: Color::RED,
					..default()
				},
			)
			.with_text_alignment(TextAlignment::Center);
			commands.spawn(text_bundle);
		});
    });

	let Ok(weapon_id) = q_player_weapon.get_single() else { return };
	commands.entity(weapon_id).despawn_recursive();
}

fn setup(
    mut commands: Commands,
    r_asset_server: Res<AssetServer>,
    mut rm_texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	//Loads the sprite sheet and specify what part of it to use
    let player_texture = r_asset_server.load("textures/rpg/chars/mani/mani-idle-run.png");
    let player_texture_atlas =
        TextureAtlas::from_grid(player_texture, Vec2::splat(24.0), 7, 1, None, None);
    let player_texture_atlas = rm_texture_atlases.add(player_texture_atlas);

	//Spawn player character
    let player_id = commands.spawn((
		bundles::PlayerBundle {
			transform: Transform::from_xyz(0.0, 0.0, 900.0),
			//components::WalkAnimate::new(25.0, 1.0, 2.0),
			walk_animate: components::WalkAnimate::new_with_marker(25.0, 1.0, 2.0, BevyDuration::from_secs_f32(0.1)),
			health: components::Health { current: 100.0, ..default() },
			..default()
		},
		components::Intersect{
			size: Vec2::splat(24.0),
		}
    ))
	.with_children(|parent| {
		parent.spawn(
			SpriteSheetBundle {
            	texture_atlas: player_texture_atlas,
            	sprite: TextureAtlasSprite::new(0),
            	..default()
			},
		);
	}).id();
	
	commands.spawn((
		NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				width: Val::Px(25.0),
				height: Val::Px(100.0),
				..default()
			},
			background_color: Color::GREEN.into(),
			..default()
		},
		components::HealthMeter { id: player_id },
	));
}
