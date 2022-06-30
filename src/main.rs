use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

#[derive(Component)]
struct Triangle {
    dx: f32,
    dy: f32,
    heading: f32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("ship5.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        })
        .insert(Triangle {
            dx: 100.,
            dy: 75.,
            heading: 0.,
        });
}

fn movement(time: Res<Time>, mut sprite_position: Query<(&mut Triangle, &mut Transform)>) {
    for (mut triangle, mut transform) in sprite_position.iter_mut() {
        if transform.translation.y > 200. || transform.translation.y < -200. {
            triangle.dy *= -1.;
        }
        if transform.translation.x > 200. || transform.translation.x < -200. {
            triangle.dx *= -1.;
        }
        transform.translation.y += triangle.dy * time.delta_seconds();
        transform.translation.x += triangle.dx * time.delta_seconds();
        transform.rotate(Quat::from_rotation_z(
            triangle.heading + time.delta_seconds(),
        ));
    }
}
