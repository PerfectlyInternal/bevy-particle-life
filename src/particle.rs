use std::{
    time::Duration,
    f32::consts::TAU,
};
use bevy:: {
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use rand::random;

use crate::asset::*;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParticleCounter(0));
        app.add_systems(Startup, spawn_emmiters);
        app.add_systems(Update, (
                emit_particles,
                cancel_collided_particles,
                delete_cancelled_particles
            ));
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Charge(pub f32);

#[derive(Component)]
struct Cancelled(bool);

#[derive(Bundle)]
pub struct Particle {
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
pub struct ParticleCounter(pub u16);

const DELETION_RADIUS: f32 = 10.0;
const SPAWN_VELOCITY: f32 = 250.0;
const MAX_PARTICLE_COUNT: u16 = 2500;
const SPAWN_TIME_MSEC: u64 = 100;
const EMMITER_COUNT: u16 = 10;

#[allow(dead_code)]
fn spawn_random_particles(
    mut commands: Commands,
    assets: Res<ParticleAssets>,
    mut counter: ResMut<ParticleCounter>,
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

        counter.0 += 1;
    }
}

fn spawn_emmiters(
    mut commands: Commands,
) {
    for _i in 0..EMMITER_COUNT {
        let positive = true;//random::<bool>();
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
            timer: Timer::new(
                       Duration::from_millis(SPAWN_TIME_MSEC),
                       TimerMode::Repeating),
        }
    );
}

fn emit_particles(
    mut commands: Commands,
    assets: Res<ParticleAssets>,
    time: Res<Time>,
    mut counter: ResMut<ParticleCounter>,
    mut emitter_timer: ResMut<EmmiterTimer>,
    q: Query<&Emmiter>,
) {
    emitter_timer.timer.tick(time.delta());
    if emitter_timer.timer.finished() && counter.0 < MAX_PARTICLE_COUNT {
        for emmiter in q.iter() {
            let dir = random::<f32>() * TAU;
            let vel = Vec2 {
                x: dir.sin() * SPAWN_VELOCITY,
                y: dir.cos() * SPAWN_VELOCITY,
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

            counter.0 += 1;
        }
    }
}

fn cancel_collided_particles(
    mut q: Query<(&mut Cancelled, &Charge, &Transform)>
) {
    let mut pairs = q.iter_combinations_mut();
    while let Some(
        [(mut cancelled_a, Charge(charge_a), transform_a),
         (mut cancelled_b, Charge(charge_b), transform_b)]
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
    mut counter: ResMut<ParticleCounter>,
    q: Query<(Entity, &Cancelled)>,
) {
    for (entity, cancelled) in q.iter() {
        if cancelled.0 {
            commands.entity(entity).despawn();
            counter.0 -= 1;
        }
    }
}
