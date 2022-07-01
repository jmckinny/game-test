use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::utils::tracing::field::debug;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(collided)
        .run();
}

#[derive(Component)]
struct Ship {
    dx: f32,
    dy: f32,
    heading: f32,
}
#[derive(Component)]
struct Bullet;

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
        .insert(Ship {
            dx: 0.,
            dy: 0.,
            heading: 90.,
        });
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("bullet.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            sprite: Sprite {
                custom_size: Some(Vec2::new(25., 25.)),
                ..default()
            },
            ..default()
        })
        .insert(Bullet);
}

const MAX_SPEED: f32 = 50.;

fn movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Ship, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut ship, mut transform) in sprite_position.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            ship.dy += 10.;
            if ship.dy > MAX_SPEED {
                ship.dy -= 10.
            }
        }
        if keyboard_input.pressed(KeyCode::S) {
            ship.dy -= 10.;
            if ship.dy < -MAX_SPEED {
                ship.dy += 10.
            }
        }
        if keyboard_input.pressed(KeyCode::A) {
            ship.dx -= 10.;
            if ship.dx < -MAX_SPEED {
                ship.dx += 10.
            }
        }
        if keyboard_input.pressed(KeyCode::D) {
            ship.dx += 10.;
            if ship.dx > MAX_SPEED {
                ship.dx -= 10.
            }
        }

        if transform.translation.y > 200. || transform.translation.y < -200. {
            ship.dy = 0.;
        }
        if transform.translation.x > 200. || transform.translation.x < -200. {
            ship.dx = 0.;
        }
        transform.translation.y += ship.dy * time.delta_seconds();
        transform.translation.x += ship.dx * time.delta_seconds();
    }
}

fn collided(
    mut ship_pos: Query<(&mut Ship, &Transform), With<Ship>>,
    mut bullets: Query<(&mut Bullet, &Transform), With<Bullet>>,
) {
    let (ship, ship_transform) = ship_pos.single_mut();
    for (mut bullet, bullet_transform) in bullets.iter_mut() {
        if collide(
            bullet_transform.translation,
            Vec2::new(25., 25.),
            ship_transform.translation,
            Vec2::new(100.,100.),
        ).is_some() {
            println!("Collision detected");
        } else{
            println!("Not colliding")
        }
    }
}
