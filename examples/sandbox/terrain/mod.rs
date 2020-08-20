use bevy::prelude::*;
use std::{hash::Hash, collections::HashSet, fmt::Debug};
use bevy::render::camera::Camera;
use bevy_math::Mat2;
use hex::*;

pub mod hex;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<hex::CubeHexLayout>()
            .init_resource::<TerrainRegistry<CubeHexCoord>>()
            .init_resource::<TerrainOptions>()
            .add_system(spawn_chunks.system())
            .add_system(load_chunks.system());
    }
}

impl Default for TerrainPlugin {
    fn default() -> Self {
        TerrainPlugin
    }
}

pub struct TerrainOptions {
    pub chunk_size: i32,
    pub voxel_size: f32,
    pub origin: ChunkIndex,
}
impl Default for TerrainOptions {
	fn default() -> Self {
		Self {
            chunk_size: 32,
            voxel_size: 32.0,
            origin: ChunkIndex::default()
		}
	}
}

pub struct TerrainRegistry<T> where T: Hash + Eq {
    pub loaded_chunks: HashSet<T>,
    pub chunks_to_load: HashSet<T>,
}
impl<T> Default for TerrainRegistry<T> where T: Hash + Eq {
    fn default() -> Self {
        TerrainRegistry{
            loaded_chunks: HashSet::new(),
            chunks_to_load: HashSet::new(),
        }
    }
}
impl<T> TerrainRegistry<T> where T: Hash + Eq + Debug {
    pub fn queue_load(&mut self, chunk: T) {
        if !self.loaded_chunks.contains(&chunk) && !self.chunks_to_load.contains(&chunk) {
            println!("Loading chunk {:?}", chunk);
            self.chunks_to_load.insert(chunk);
        }
    }
    pub fn queue_load_all(&mut self, chunks: impl Iterator<Item=T>) {
        for chunk in chunks {
            self.queue_load(chunk)
        }
    }
    pub fn mark_loaded(&mut self, chunk: T) {
        self.loaded_chunks.insert(chunk);
    }
    pub fn mark_loaded_all(&mut self, chunks: impl Iterator<Item=T>) {
        self.loaded_chunks.extend(chunks)
    }
}

fn spawn_chunks(
    hex_layout: Res<CubeHexLayout>,
    mut terrain_registry: ResMut<TerrainRegistry<CubeHexCoord>>,
    mut query: Query<(&Translation, &ChunkSite)>,
) {
    // load chunks around cameras
    for (translation, _site) in &mut query.iter() {
        // find which chunk we're currently on
        let current_chunk = hex_layout.space_to_hex(Vec2::new(translation.x(), translation.y()));
        // find neighboring chunks
        let neighbors = hex_layout.get_neighbors(current_chunk);

        // load chunks
        terrain_registry.queue_load(current_chunk);
        terrain_registry.queue_load_all(neighbors);
    }

    // create entities for chunks
}

fn load_chunks(
    mut commands: Commands,
    hex_layout: Res<CubeHexLayout>,
    mut terrain_registry: ResMut<TerrainRegistry<CubeHexCoord>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // enumerate chunks that needs to be loaded
    let loaded: Vec<_>=
        terrain_registry
        .chunks_to_load.drain().map(|chunk| {
        // TODO: check if there is any persisted chunk state
        // TODO: if yes, load from disk
        // if no, procedurally generate chunk
        // loading a chunk might need multiple cycles
        // once completely loaded, mark the chunk as loaded

        let pos = hex_layout.hex_to_space(chunk);
        commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
            sprite: Sprite { size: Vec2::from_slice_unaligned(&[50.0, 50.0]) },
            translation: Translation::new(pos.x(), pos.y(), 1.0),
            ..Default::default()
        });
        chunk
    }).collect();

    terrain_registry.mark_loaded_all(loaded.into_iter());
}

fn unload_chunks() {
    // find chunks that can be unloaded
    // mark them for despawning
}

fn despawn_chunks() {
    // find chunks marked for despawning
    // save chunk data to disk
    // despawn chunks
}

trait ChunkLayout {
    fn get_chunk_index_from_translation(&self, origin: ChunkIndex, translation: Translation);
}

fn get_chunks_to_load(origin: ChunkIndex, camera_location: Translation, options: TerrainOptions) -> Vec<ChunkIndex> {
    // let chunk_size = options.chunk_size as f32 * options.voxel_size;
    // let half_chunk_size = chunk_size / 2.;
    // let lon = ((translation.x + half_chunk_size) / chunk_size).floor() as i32;
    // let lat = ((translation.z + half_chunk_size) / chunk_size).floor() as i32;
    todo!()
}

#[derive(Debug, PartialEq, Copy, Clone, Properties)]
pub struct ChunkIndex(pub i32, pub i32);

impl Default for ChunkIndex {
    fn default() -> Self {
        ChunkIndex(0, 0)
    }
}

pub struct ChunkSite {
    pub load_distance: i32
}

impl Default for ChunkSite {
    fn default() -> Self {
        ChunkSite {
            load_distance: 1
        }
    }
}

#[derive(Bundle)]
pub struct TerrainChunkComponents {
    pub pbr: PbrComponents,
    pub chunk_index: ChunkIndex,
}
