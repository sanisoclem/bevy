use bevy::prelude::*;
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
    // Add bricks
    let brick_rows = 4;
    let brick_columns = 5;
    let brick_spacing = 20.0;
    let brick_size = Vec2::new(150.0, 30.0);
    let bricks_width = brick_columns as f32 * (brick_size.x() + brick_spacing) - brick_spacing;
    // center the bricks and move them up a bit
    let bricks_offset = Vec3::new(-(bricks_width - brick_size.x()) / 2.0, 100.0, 0.0);

    for row in 0..brick_rows {
        let y_position = row as f32 * (brick_size.y() + brick_spacing);
        for column in 0..brick_columns {
            let brick_position = Vec3::new(
                column as f32 * (brick_size.x() + brick_spacing),
                y_position,
                0.0,
            ) + bricks_offset;
            commands
                // brick
                .spawn(SpriteComponents {
                    material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
                    sprite: Sprite { size: brick_size },
                    translation: Translation(brick_position),
                    ..Default::default()
                });
        }
    }

    let texture_handle = asset_server.load("assets/branding/icon.png").unwrap();
    commands
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
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

        *translation.0.x_mut() += time.delta_seconds * direction_x * 150.0;
        *translation.0.y_mut() += time.delta_seconds * direction_y * 150.0;
    }
}