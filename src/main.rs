use std::f32::consts::PI;
use std::ops::Rem;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor{
            title: "Game Test".to_string(),
            ..default()
        })
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

#[derive(Component)]
struct Ship {
    dx: f32,
    dy: f32,
    spin: f32,
    angle:f32
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
            spin: 0.,
            angle: 90.
        });
}

const MAX_SPEED: f32 = 300.;
const ACCEL: f32 = 300.;
const MAX_SPIN:f32 = 10.;
const SPIN: f32 = 10.;
fn movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Ship, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut ship, mut transform) in sprite_position.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            ship.dx += (ship.angle*PI/180.).cos()*ACCEL*time.delta_seconds();
            ship.dy += (ship.angle*PI/180.).sin()*ACCEL*time.delta_seconds();
            //println!("Heading: {} dx:{} dy:{}",ship.angle, ship.dx,ship.dy);
        }
        if keyboard_input.pressed(KeyCode::A) {
            ship.spin += SPIN * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            ship.spin -= SPIN * time.delta_seconds();
        }

        if ship.dx > MAX_SPEED {
            ship.dx = MAX_SPEED;
        } else if ship.dx < -MAX_SPEED {
            ship.dx = -MAX_SPEED
        }

        if ship.dy > MAX_SPEED {
            ship.dy = MAX_SPEED;
        } else if ship.dy < -MAX_SPEED {
            ship.dy = -MAX_SPEED
        }

        if ship.spin > MAX_SPIN {
            ship.spin = MAX_SPIN;
        } else if ship.spin < -MAX_SPIN {
            ship.spin = -MAX_SPIN
        }
        let offset = ship.spin * time.delta_seconds();
        ship.angle = (ship.angle + (offset * (180./PI))).rem(360.);
        transform.translation.y += ship.dy * time.delta_seconds();
        transform.translation.x += ship.dx * time.delta_seconds();
        transform.rotate(Quat::from_rotation_z(offset));
    }
}