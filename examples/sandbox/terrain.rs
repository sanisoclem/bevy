use bevy::prelude::*;
use bevy::render::camera::Camera;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<TerrainOptions>()
            .add_system(spawn_chunks.system());
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

fn spawn_chunks(
    mut commands: Commands,
    options: Res<TerrainOptions>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Transform, &Camera)>,
) {
    // load chunks around cameras
    for (transform, camera) in &mut query.iter() {

    }
    // determine which chunks needs to be spawned
    // create entities for chunks
}

fn load_chunks() {
    // enumerate chunks that needs to be loaded
    // check if there is any persisted chunk state
    // if yes, load from disk
    // if no, procedurally generate chunk
    // loading a chunk might need multiple cycles
    // once completely loaded, mark the chunk as loaded
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

fn get_chunks_to_load(origin: ChunkIndex, camera_location: Translation) -> Vec<ChunkIndex> {

}

#[derive(Debug, PartialEq, Copy, Clone, Properties)]
pub struct ChunkIndex(pub i32, pub i32);

impl Default for ChunkIndex {
    fn default() -> Self {
        ChunkIndex(0, 0)
    }
}

#[derive(Bundle)]
pub struct TerrainChunkComponents {
    pub pbr: PbrComponents,
    pub chunk_index: ChunkIndex,
}
