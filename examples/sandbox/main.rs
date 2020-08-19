use bevy::prelude::*;

mod top_down;
mod terrain;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_plugin(terrain::TerrainPlugin::default())
        .add_plugin(top_down::TopDownPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
            ..Default::default()
        })
        // cube
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            translation: Translation::new(0.0, 1.0, 0.0),
            ..Default::default()
        })
        // sphere
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.5,
            })),
            material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
            translation: Translation::new(1.5, 1.5, 1.5),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        })
        // camera
        .spawn(top_down::TopDownCamera::create_facing(
            Vec3::new(-3.0, 5.0, 8.0),
			Vec3::new(0.0, 0.0, 0.0),
			Vec3::new(0.0, 1.0, 0.0),
        ));
}


// fn cube_mover_system(
// 	time: Res<Time>,
// 	mut query: Query<(&TopDownCameraOptions, &mut Translation, &Rotation)>,
// ) {
// 	let axis_h = movement_axis(&keyboard_input, KeyCode::D, KeyCode::A);
// 	let axis_v = movement_axis(&keyboard_input, KeyCode::S, KeyCode::W);

// 	let axis_float =
// 		movement_axis(&keyboard_input, KeyCode::Space, KeyCode::LShift);

// 	for (options, mut translation, rotation) in &mut query.iter() {
// 		let delta_f = forward_walk_vector(&rotation) // Vec3::unit_z().normalize()
// 			* axis_v
// 			* options.speed
// 			* time.delta_seconds;

// 		let delta_strafe =
// 			strafe_vector(rotation) * axis_h * options.speed * time.delta_seconds;

// 		let delta_float =
// 			Vec3::unit_y() * axis_float * options.speed * time.delta_seconds;

// 		translation.0 += delta_f + delta_strafe + delta_float;
// 	}
// }
