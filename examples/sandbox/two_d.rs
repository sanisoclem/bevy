use bevy::prelude::*;
use bevy_math::Mat2;
use crate::terrain::ChunkSite;

pub struct TwoDPlugin;

impl Plugin for TwoDPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup2d.system())
            .add_system(sprite_movement.system());
    }
}

impl Default for TwoDPlugin {
    fn default() -> Self {
        TwoDPlugin
    }
}


fn setup2d(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Add hexes
    let hex_distance = 5;
    let size = 100.0;
    let hex2pixel = Mat2::from_cols_array(&[3.0f32.sqrt(), 0.0, 3.0f32.sqrt()/2.0, 3.0/2.0]);

    for d in 0..=hex_distance {
        println!("ring: {}", d);
        for i in 0..=d {
            let indexes = [i, d-i, -d];
            // rotate 6 times
            for a in 0..6 {
                let m = if a % 2 == 1 { -1.0 } else { 1.0 };
                let xi = (0 + a) % 3;
                let yi = (1 + a) % 3;
                let qr = Vec2::new(indexes[xi] as f32 * m , indexes[yi] as f32 * m);

                println!("Spawning {}", qr);
                let pos = hex2pixel.mul_vec2(qr) * size;
                commands
                .spawn(SpriteComponents {
                    material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
                    sprite: Sprite { size: Vec2::from_slice_unaligned(&[10.0, 10.0]) },
                    translation: Translation::new(pos.x(), pos.y(), 1.0),
                    ..Default::default()
                });
            }
        }
    }

    let texture_handle = asset_server.load("assets/branding/icon.png").unwrap();
    commands
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            translation: Translation::new(0.0, 0.0,0.0),
            ..Default::default()
        })
        .with(ChunkSite::default())
        .with_children(|parent| {
            parent.spawn(Camera2dComponents::default());
        });
}

fn sprite_movement (
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Translation, &ChunkSite)>,
) {
	for (mut translation, _site) in &mut query.iter() {
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;
        if keyboard_input.pressed(KeyCode::A) {
            direction_x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction_x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction_y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction_y -= 1.0;
        }

        *translation.0.x_mut() += time.delta_seconds * direction_x * 550.0;
        *translation.0.y_mut() += time.delta_seconds * direction_y * 550.0;
    }
}