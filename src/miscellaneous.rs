use bevy::prelude::*;
use bevy::sprite::collide_aabb;
use bevy_particle_systems::*;

use crate::system;
use crate::components;
use crate::events;
use crate::bundles;

use bevy::utils::Duration as BevyDuration;


#[derive(Clone, Default)]
pub struct MiscellaneousPlugin;

impl Plugin for MiscellaneousPlugin {
	fn build(&self, app: &mut App) {
		app
		//why does it need to be empty to execute it?
		//events are not consistantly cleared in each frame, if checking is executed each frame it will
		//inconsistantly stack intersection events
		//it is possibly to control the clearing of events but using run_if is fine considering the scope of this project
		.add_systems(Update, check_intersect.run_if(intersect_empty()).in_set(system::GameSet::Check))
		.add_systems(Update, (do_walk_animation, move_with_velocity).chain().in_set(system::GameSet::Apply))
		.add_systems(Update, despawn_by_timer.in_set(system::GameSet::Despawn))
		.add_systems(Update, health_meter.in_set(system::GameSet::CommandsAction))
		.add_systems(Update, (damage_zone_apply, damage_apply).in_set(system::GameSet::Apply));
	}
}

//No need to use Condition just use bevy's implementation, it is faster
fn intersect_empty() -> impl Fn(EventReader<events::IntersectEvent>) -> bool + Clone {
	move |reader: EventReader<events::IntersectEvent>| { reader.is_empty() }
}

//Will apply the unapplied_damage and create blood particles
fn damage_apply(
	mut commands: Commands,
	mut q_health: Query<(&Transform, &mut components::Health)>
) {
	for (transform, mut health) in q_health.iter_mut() {
		if health.unapplied_damage == 0.0 {
			continue;
		}
		generic_particle_burst(
			&mut commands,
			transform.translation,
			100.0,
			0.3,
			1.0,
			10,
			(Color::RED..Color::rgba(1.0, 0.0, 0.0, 0.0)).into()
		);
		health.current -= health.unapplied_damage;

		health.unapplied_damage = 0.0;
	}
}

//Update health meter height according to the assigned entity current health
//Will also despawn it if no entity matches the assigned id
fn health_meter(
	mut commands: Commands,
	mut q_style: Query<(Entity, &mut Style, &components::HealthMeter)>,
	q_health: Query<&components::Health>,
) {
	for (id, mut style, health_meter) in q_style.iter_mut() {
		let Ok(health) = q_health.get(health_meter.id) else {
			commands.entity(id).despawn_recursive();
			continue;
		};
		style.height = Val::Px(health.current * 5.0);
	}
}

fn do_walk_animation(
	mut commands: Commands,
	mut q_walk_animators: Query<(&mut components::WalkAnimate, &components::Velocity)>,
	mut q_visual_comps: Query<(&mut Transform, &GlobalTransform, &mut TextureAtlasSprite, &Handle<TextureAtlas>, &Parent)>,
	time: Res<Time>,
) {
	for (
		mut transform,
		g_transform,
		mut texture_atlas_sprite,
		texture_atlas_handle, parent,
	) in q_visual_comps.iter_mut() {
		//match parent and child
		let Ok((mut values, velocity)) = q_walk_animators.get_mut(parent.get()) else { continue; };

		if velocity.v == Vec2::ZERO {
			continue;
		} //is there is no movement, the code below will not be executed for the current sprite and will jump to the next

		//if there is any movement to the left the sprite will be flipped
		let should_flip = velocity.v.x < 0.0;
		texture_atlas_sprite.flip_x = should_flip;

		//do a little angle and positioning movement to animate walking
		let angle = f32::sin(time.elapsed_seconds() * values.spd);
		transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, values.intensity * angle * 0.1);
		transform.translation = Vec3::new(-angle * values.step_intensity, 0.0, 0.0);

		if let Some(timer) = &mut values.ghosting_timer {
			timer.tick(time.delta());

			if timer.finished() {
				//This could be optimzed with object pooling,but it is way beyond the scope of this project
				commands.spawn(
					bundles::GhostedSpriteBundle::new(
						BevyDuration::from_secs_f32(0.4), 
						g_transform.compute_transform(),
						texture_atlas_handle.clone(),
						should_flip
					)
				);
			}
		} //Ghosting Marking Effect
	}
}

fn damage_zone_apply(
	mut er_intersect: EventReader<events::IntersectEvent>,
	mut q_health    : Query<(Entity, &mut components::Health), With<components::Intersect>>,
	q_zone          : Query<(Entity, &components::DamageZone), With<components::Intersect>>,
) {
	for event in er_intersect.read() {
		let (a, b) = event.ab;
		let health_result = q_health.get_component_mut::<components::Health>(a);
		let zone_result   = q_zone.get_component::<components::DamageZone>(b);
		let (Ok(mut health), Ok(zone)) = (health_result, zone_result) else { return };
		health.unapplied_damage += zone.damage;
	}
}

fn check_intersect(
	mut ew_intersect: EventWriter<events::IntersectEvent>,
	q_intersect: Query<(Entity, &components::Intersect, &Transform)>,
) {
	for (id_a, intersect_a, transform_a) in q_intersect.iter() {
		for (id_b, intersect_b, transform_b) in q_intersect.iter() {
			if id_a == id_b {
				continue;
			}
			let collide = collide_aabb::collide(transform_a.translation, intersect_a.size,
				transform_b.translation, intersect_b.size);

			if let Some(_result) = collide {
				ew_intersect.send(events::IntersectEvent{ab: (id_a, id_b)});
			}
		}
	}
}

const BOUNDS: f32 = 170.0;

//Each entity transform is moved according to it's velocity value and the value is reset
fn move_with_velocity(
	mut query: Query<(&mut Transform, &mut components::Velocity)>,
	r_time: Res<Time>,
) {
	//Using delta seconds here is not optimal but is fine
	let dt = r_time.delta_seconds();
	for (mut transform, mut vel) in query.iter_mut() {
		let mut translation = transform.translation;
		let z_order = translation.z;
		translation += Vec3::new(vel.v.x * dt, vel.v.y * dt, 0.0);

		//bounds
		let extents = Vec3::from((Vec2::splat(BOUNDS * 2.0), 0.0));
		translation = translation.min(extents).max(-extents);
		translation.z = z_order;

		transform.translation = translation;
		vel.v = Vec2::ZERO;
	}
}

fn despawn_by_timer(
	mut commands: Commands,
	mut query: Query<(Entity, &mut components::DespawnerTimer)>,
	r_time: Res<Time>,
) {	
	for (id, mut timer) in query.iter_mut() {
		timer.timer.tick(r_time.delta());

		if timer.timer.finished() {
			commands.entity(id).despawn();
		}
	}
}

pub fn generic_particle_burst(
	commands: &mut Commands,
	translation: Vec3,
	speed: f32,
	lifetime: f32,
	scale: f32,
	count: usize,
	color: ColorOverTime,
) -> Entity {
	let e = commands.spawn((
		ParticleSystemBundle {
			transform: Transform::from_translation(translation),
			particle_system: ParticleSystem {
				spawn_rate_per_second: 0.0.into(),
				max_particles: 1000,
				initial_speed: (0.0..speed).into(),
				lifetime: lifetime.into(),
				scale: scale.into(),
				color,
				bursts: vec![ParticleBurst {
					time: 0.0,
					count,
				}],
				..ParticleSystem::oneshot()
			},
			..default()
		},
		Playing,
		components::DespawnerTimer::new(bevy::utils::Duration::from_secs_f32(lifetime)),
	)).id();

	e
}
