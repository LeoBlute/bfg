use bevy::prelude::*;
use bevy_simple_tilemap::prelude::*;
use rand::prelude::*;

#[derive(Clone, Default)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, generate_map);
		app.add_plugins(SimpleTileMapPlugin);
	}
}

const SPRITE_DIVISION: usize = 16;
const TILE_SIZE:       usize = 16 / SPRITE_DIVISION;
const MAP_SIZE:        isize = 200;
//max is[SPRITE_DIVISION * SPRITE_DIVISION - 1], the amount of blocks in the sprite that can be used in tiles
//how many are used will affect the look of the map
const BLOCK_RANGE:     usize = 50;

fn generate_map(
	mut commands: Commands,
	r_asset_server: Res<AssetServer>,
	mut rm_texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	let texture_handle = r_asset_server.load("textures/rpg/tiles/generic-rpg-tile02.png");
    let texture_atlas =
        TextureAtlas::from_grid(
			texture_handle,
			Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
			SPRITE_DIVISION, SPRITE_DIVISION,
			None, None
		);
    let texture_atlas_handle = rm_texture_atlases.add(texture_atlas);

	let mut rng = rand::thread_rng();
	let mut tiles: Vec<(IVec3, Option<Tile>)> = Vec::new();

	for x in -MAP_SIZE..MAP_SIZE {
		for y in -MAP_SIZE..MAP_SIZE {
			let index = rng.gen_range(0..BLOCK_RANGE);
			tiles.push((
				IVec3::new(x as i32, y as i32, 0),
				Some(Tile { sprite_index: index as u32, color: Color::WHITE, ..default() })
				));
		}
	}

	let mut tilemap = TileMap::default();
	tilemap.set_tiles(tiles);

	commands.spawn(
		TileMapBundle {
			transform: Transform::from_scale(Vec3::splat(3.0)),
			tilemap,
			texture_atlas: texture_atlas_handle.clone(),
			..default()
		}
	);

}
