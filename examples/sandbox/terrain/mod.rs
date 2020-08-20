use bevy::prelude::*;
use std::{hash::Hash, collections::HashSet, fmt::Debug};
use bevy::render::camera::Camera;
use bevy_math::Mat2;
use hex::*;
use bevy_render::{mesh::{VertexAttributeValues, VertexAttribute}, pipeline::PrimitiveTopology};

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
    pub origin: CubeHexCoord,
}
impl Default for TerrainOptions {
	fn default() -> Self {
		Self {
            chunk_size: 32,
            voxel_size: 32.0,
            origin: CubeHexCoord::default()
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
        let current_chunk = hex_layout.space_to_hex(Vec2::new(translation.x(), translation.z()));
        // find neighboring chunks
        let neighbors = hex_layout.get_neighbors(current_chunk);

        // load chunks
        terrain_registry.queue_load(current_chunk);
        terrain_registry.queue_load_all(neighbors);
    }

    // create entities for chunks
}

fn chunk_to_mesh(layout: &Res<CubeHexLayout>, chunk: CubeHexCoord) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let start = Vec2::new(0.0, 1.0);
    let numVerts = 6;

    // compute vertices
    let vertices=
        (0..numVerts).map(|rot| (rot as f32 * 60.0).to_radians())
        .map(|rot| Mat2::from_cols_array(&[rot.to_radians().cos(), rot.sin(), -rot.sin(), rot.cos()]).mul_vec2(start)*layout.size)
        .map(|v2| [v2.x(), 0.0, v2.y()]);

    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Position".into(),
        values: VertexAttributeValues::Float3(vertices.collect()),
    });

    // compute normals
    let normals=
        (0..numVerts).map(|_| Vec3::unit_y().normalize().into());

    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Normal".into(),
        values: VertexAttributeValues::Float3(normals.collect()),
    });

     // compute UVs
     let uvs=
     (0..numVerts).map(|_| [0.0, 0.0]);

    mesh.attributes.push(VertexAttribute {
        name: "Vertex_Uv".into(),
        values: VertexAttributeValues::Float2(uvs.collect()),
    });

    // indices
    mesh.indices = Some(vec![0,1,1,2,2,3,3,4,4,5,5,0]);
    //mesh.indices = Some(vec![5,0,1,2,3,4,5,2,3,3]);
    mesh
}

fn load_chunks(
    mut commands: Commands,
    hex_layout: Res<CubeHexLayout>,
    mut terrain_registry: ResMut<TerrainRegistry<CubeHexCoord>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        // commands
        // .spawn(SpriteComponents {
        //     material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
        //     sprite: Sprite { size: Vec2::from_slice_unaligned(&[50.0, 50.0]) },
        //     translation: Translation::new(pos.x(), pos.y(), 1.0),
        //     ..Default::default()
        // });

        commands.spawn(PbrComponents {
            mesh: meshes.add(chunk_to_mesh(&hex_layout, chunk) ),
            material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
            translation: Translation::new(pos.x(), 0.0, pos.y()),
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
