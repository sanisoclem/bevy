use bevy::prelude::*;
use bevy_math::Mat2;
use crate::terrain::{ChunkSite, hex::{CubeHexLayout,HexLayout,CubeHexCoord}};

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
    hex_layout: Res<CubeHexLayout>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Add hexes
    let hex_distance = 5;
    let center = CubeHexCoord::default();
    let material = materials.add(Color::rgb(0.2, 0.2, 0.8).into());

    for d in 0..=hex_distance {
        println!("ring: {}", d);
        for hex in hex_layout.get_ring(center, d) {
            println!("Spawning {:?}", hex);
            let pos = hex_layout.hex_to_space(hex);
            commands
            .spawn(SpriteComponents {
                material: material,
                sprite: Sprite { size: Vec2::from_slice_unaligned(&[10.0, 10.0]) },
                translation: Translation::new(pos.x(), pos.y(), 1.0),
                ..Default::default()
            });
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