use bevy::prelude::*;
use rand::prelude::*;

use crate::components;

#[derive(Bundle, Default, Clone)]
pub struct GhostedSpriteBundle {
	pub despawner          : components::DespawnerTimer,
	pub sprite_sheet_bundle: SpriteSheetBundle,
}

#[derive(Bundle, Default, Clone)]
pub struct PlayerBundle {
    pub visibility          : Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility     : ViewVisibility,
    pub transform           : Transform,
    pub global_transform    : GlobalTransform,
	pub walk_animate        : components::WalkAnimate,
	pub player_char         : components::PlayerCharacter,
	pub velocity            : components::Velocity,
	pub health              : components::Health,
}

impl GhostedSpriteBundle {
	pub fn new(
		disappear_time: bevy::utils::Duration,
		transform     : Transform,
		texture_atlas : Handle<TextureAtlas>,	
		flip_x        : bool
	) -> Self{
		let mut rng = rand::thread_rng();
		let r = rng.gen_range(0.0..1.0);
		let g = rng.gen_range(0.0..1.0);
		let b = rng.gen_range(0.0..1.0);
		let a = rng.gen_range(0.7..0.8);
		Self {
			despawner: components::DespawnerTimer::new(disappear_time),
			sprite_sheet_bundle: SpriteSheetBundle {
				transform,
				texture_atlas,
				sprite: TextureAtlasSprite {
					flip_x,
					color: Color::rgba(r, g, b, a),
					..default()
				},
				..default()
			}
		}
	}
}
