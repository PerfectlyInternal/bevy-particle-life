use bevy:: {
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use std::{
    time::Duration,
    f32::consts::TAU,
};

use rand::random;

use crate::{
    particle::*,
    asset::ParticleAssets,
};

pub struct EmmiterPlugin;

impl Plugin for EmmiterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_emmiters);
        app.add_systems(Update, emit_particles);
    }
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

const EMMITER_COUNT: u16 = 10;
const EMMITER_SPAWN_RANGE: f32 = 1000.0;
const SPAWN_VELOCITY: f32 = 250.0;
const SPAWN_TIME_MSEC: u64 = 100;
const MAX_PARTICLE_COUNT: u16 = 2500;

fn spawn_emmiters(
    mut commands: Commands,
) {
    for _i in 0..EMMITER_COUNT {
        let positive = true;//random::<bool>();
        let x = random::<f32>() * EMMITER_SPAWN_RANGE - EMMITER_SPAWN_RANGE/2.0;
        let y = random::<f32>() * EMMITER_SPAWN_RANGE - EMMITER_SPAWN_RANGE/2.0;

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
                ParticleBundle {
                    velocity: Velocity(vel),
                    charge: Charge(emmiter.charge.0),
                    cancelled: Cancelled(false),
                    particle: Particle
                },
                MaterialMesh2dBundle {
                    mesh: assets.circle.clone(),
                    material: if emmiter.charge.0 > 0.0 { assets.red.clone() } else { assets.blue.clone() },
                    transform: emmiter.transform, 
                    ..default()
                }
            ));

            counter.0 += 1;
        }
    }
}
