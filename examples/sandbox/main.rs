use bevy::prelude::*;
use terrain::ChunkSite;

mod debug;
mod terrain;
mod top_down;
mod two_d;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_plugin(terrain::TerrainPlugin::default())
        .add_plugin(top_down::TopDownPlugin::default())
        .add_plugin(debug::DebugPlugin::default())
        //.add_plugin(two_d::TwoDPlugin::default())
        .add_startup_system(setup3d.system())
        .run();
}

fn setup3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // plane
        // .spawn(PbrComponents {
        //     mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        //     material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
        //     ..Default::default()
        // })
        // cube
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            translation: Translation::new(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .spawn(LightComponents {
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        })
        // sphere
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.5,
            })),
            material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
            translation: Translation::new(0.0, 2.0, 0.0),
            ..Default::default()
        })
        .with(top_down::TopDownCameraOptions::default())
        .with(ChunkSite::default())
        .with_children(|parent| {
            parent
                // camera
                .spawn(top_down::TopDownCamera::create_facing(
                    Vec3::new(0.0, 5.0, 8.0),
                    Vec3::new(0.0, 2.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                ));
        });
}
