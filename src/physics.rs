use bevy::prelude::*;

use crate::particle::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            (apply_particle_forces,
             apply_particle_velocities).chain());
    }
}

const K: f32 = 1000000.0;

fn apply_particle_forces(
    time: Res<Time>,
    mut q: Query<(&mut Velocity, &Charge, &Transform)>
) {
    let mut pairs = q.iter_combinations_mut();
    while let Some(
        [(mut velocity_a, Charge(charge_a), transform_a),
         (mut velocity_b, Charge(charge_b), transform_b)]
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
