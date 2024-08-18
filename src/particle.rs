
use bevy:: {
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use rand::random;

use crate::{
    asset::ParticleAssets,
    emmiter::EmmiterPlugin,
};

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ParticleCounter(0));
        app.add_plugins(EmmiterPlugin);
        app.add_systems(Update, (
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
pub struct Cancelled(pub bool);

#[derive(Component)]
pub struct Particle;

#[derive(Bundle)]
pub struct ParticleBundle {
    pub velocity: Velocity,
    pub charge: Charge,
    pub cancelled: Cancelled,
    pub particle: Particle
}

#[derive(Resource)]
pub struct ParticleCounter(pub u16);

const DELETION_RADIUS: f32 = 10.0;

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
            ParticleBundle {
                velocity: Velocity(Vec2::ZERO),
                charge: if positive { Charge(1.0) } else { Charge(-1.0) },
                cancelled: Cancelled(false),
                particle: Particle
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
