use std::time::Duration;
use bevy:: {
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},

};
use rand::random;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, generate_assets);
        app.add_systems(Startup, (spawn_random_particles, spawn_emmiters));
        app.add_systems(Update, (apply_particle_forces, apply_particle_velocities).chain());
        app.add_systems(Update, (
                emit_particles,
                cancel_collided_particles,
                delete_cancelled_particles
            ));
    }
}

#[derive(Component)]
#[allow(dead_code)]
enum ParticleType {
    RED,
    GREEN,
    BLUE
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Charge(f32);

#[derive(Component)]
struct Cancelled(bool);

#[derive(Bundle)]
struct Particle {
    velocity: Velocity,
    charge: Charge,
    cancelled: Cancelled,
}

#[derive(Component)]
struct Emmiter {
    transform: Transform,
    charge: Charge,
}

#[derive(Resource)]
struct EmmiterTimer {
    timer: Timer
}

#[derive(Resource)]
struct ParticleAssets {
    circle: Mesh2dHandle,
    red: Handle<ColorMaterial>,
    blue: Handle<ColorMaterial>
}

const K: f32 = 250000.0; // 9000000000.0;
const DELETION_RADIUS: f32 = 10.0;
const SPAWN_VELOCITY: f32 = 250.0;

fn generate_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(
        ParticleAssets {
            circle: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            red: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            blue: materials.add(Color::srgb(0.0, 0.0, 1.0)),
        }
    );
}

fn spawn_random_particles(
    mut commands: Commands,
    assets: Res<ParticleAssets>,
) { 
    for _i in 0..100 {
        let positive = random::<bool>();
        let x = random::<f32>() * 1000.0;
        let y = random::<f32>() * 1000.0;

        commands.spawn((
            Particle {
                velocity: Velocity(Vec2::ZERO),
                charge: if positive { Charge(1.0) } else { Charge(-1.0) },
                cancelled: Cancelled(false)
            },
            MaterialMesh2dBundle {
                mesh: assets.circle.clone(),
                material: if positive { assets.red.clone() } else { assets.blue.clone() },
                transform: Transform::from_xyz(x, y, 0.0), 
                ..default()
            }
        ));
    }
}

fn spawn_emmiters(
    mut commands: Commands,
) {
    for _i in 0..5 {
        let positive = random::<bool>();
        let x = random::<f32>() * 1000.0;
        let y = random::<f32>() * 1000.0;

        commands.spawn(
            Emmiter {
                transform: Transform::from_xyz(x, y, 0.0),
                charge: if positive { Charge(1.0) } else { Charge(-1.0) },
            }
        );
    }

    commands.insert_resource(
        EmmiterTimer {
            timer: Timer::new(Duration::from_millis(250), TimerMode::Repeating),
        }
    );
}

fn emit_particles(
    mut commands: Commands,
    assets: Res<ParticleAssets>,
    time: Res<Time>,
    mut emitter_timer: ResMut<EmmiterTimer>,
    q: Query<&Emmiter>,
) {
    emitter_timer.timer.tick(time.delta());
    if emitter_timer.timer.finished() {
        for emmiter in q.iter() {
            let vel = Vec2 {
                x: (random::<f32>() * SPAWN_VELOCITY * 2.0) - (SPAWN_VELOCITY),
                y: (random::<f32>() * SPAWN_VELOCITY * 2.0) - (SPAWN_VELOCITY)
            };
            commands.spawn((
                Particle {
                    velocity: Velocity(vel),
                    charge: Charge(emmiter.charge.0),
                    cancelled: Cancelled(false),
                },
                MaterialMesh2dBundle {
                    mesh: assets.circle.clone(),
                    material: if emmiter.charge.0 > 0.0 { assets.red.clone() } else { assets.blue.clone() },
                    transform: emmiter.transform.clone(), 
                    ..default()
                }
            ));
        }
    }
}

fn apply_particle_forces(
    time: Res<Time>,
    mut q: Query<(&mut Velocity, &Charge, &Transform)>
) {
    let mut pairs = q.iter_combinations_mut();
    while let Some(
        [(mut velocity_a, Charge(charge_a), transform_a), (mut velocity_b, Charge(charge_b), transform_b)]
    ) = pairs.fetch_next() {
        let delta = transform_a.translation - transform_b.translation;
        let direction = delta.normalize();
        let distance = delta.length();
        let force = K * ((charge_a * charge_b) / f32::powf(distance, 2.0));
        velocity_a.0 += direction.xy() * force * time.delta_seconds();
        velocity_b.0 += -direction.xy() * force * time.delta_seconds();
    }
}

fn apply_particle_velocities(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Velocity)>
) {
    for (mut transform, velocity) in &mut q {
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    }
}

fn cancel_collided_particles(
    mut q: Query<(&mut Cancelled, &Charge, &Transform)>
) {
    let mut pairs = q.iter_combinations_mut();
    while let Some(
        [(mut cancelled_a, Charge(charge_a), transform_a), (mut cancelled_b, Charge(charge_b), transform_b)]
    ) = pairs.fetch_next() {
        let distance = transform_a.translation.distance(transform_b.translation);
        if charge_a * charge_b < 0.0 && distance < DELETION_RADIUS {
            cancelled_a.0 = true;
            cancelled_b.0 = true;
        }
    }
}

fn delete_cancelled_particles(
    mut commands: Commands,
    q: Query<(Entity, &Cancelled)>,
) {
    for (entity, cancelled) in q.iter() {
        if cancelled.0 {
            commands.entity(entity).despawn();
        }
    }
}
