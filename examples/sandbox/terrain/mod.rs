use bevy::prelude::*;
use std::collections::HashSet;
use bevy::render::camera::Camera;
use bevy_math::Mat2;

mod hex;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TerrainRegistry>()
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

pub struct TerrainRegistry {
    pub loaded_chunks: HashSet<(i32, i32)>,
    pub chunks_to_load: HashSet<(i32, i32)>,
}
impl Default for TerrainRegistry {
    fn default() -> Self {
        TerrainRegistry{
            loaded_chunks: HashSet::new(),
            chunks_to_load: HashSet::new(),
        }
    }
}

fn spawn_chunks(
    mut commands: Commands,
    options: Res<TerrainOptions>,
    mut terrain_registry: ResMut<TerrainRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Translation, &ChunkSite)>,
) {
    let pixel2hex = Mat2::from_cols_array(&[3.0f32.sqrt()/3.0, 0.0, -1.0/3.0, 2.0/3.0]);
    // load chunks around cameras
    for (translation, site) in &mut query.iter() {
        // // find which chunk we're currently on
        // let pos = Vec2::from_slice_unaligned(&[translation.x(), translation.y()]);
        // let qr = pixel2hex.mul_vec2(pos) / size;

        // for d in 0..site.load_distance {
        //     for i in 0..=d {
        //         let indexes = [i, d-i, -d];
        //         // rotate 6 times
        //         for a in 0..6 {
        //             let m = if a % 2 == 1 { -1.0 } else { 1.0 };
        //             let xi = (0 + a) % 3;
        //             let yi = (1 + a) % 3;
        //             let qr = Vec2::new(indexes[xi] as f32 * m , indexes[yi] as f32 * m);

        //             println!("Spawning {}", qr);
        //             let pos = hex2pixel.mul_vec2(qr) * size;
        //             commands
        //             .spawn(SpriteComponents {
        //                 material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
        //                 sprite: Sprite { size: Vec2::from_slice_unaligned(&[10.0, 10.0]) },
        //                 translation: Translation::new(pos.x(), pos.y(), 1.0),
        //                 ..Default::default()
        //             });
        //         }
        //     }
        // }


        // // create list of chunks that has to be loaded
        // print!("{:?}", qr);
        // print!("{:?}, {:?}", q, r);
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
