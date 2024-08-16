use bevy::prelude::*;

use crate::particle::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            (apply_particle_forces,
             border_interaction,
             limit_speed,
             apply_particle_velocities).chain());
    }
}

const K: f32 = 1000000.0;
const BORDER_DISTANCE: f32 = 1000.0;
const MAX_SPEED: f32 = 1000.0;

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

fn border_interaction(
    mut q: Query<(&mut Velocity, &Transform)>
) {
    q.par_iter_mut().for_each(|(mut velocity, transform)| {
        if transform.translation.x > BORDER_DISTANCE {
            velocity.0.x = velocity.0.x.copysign(-1.0);
        } else if transform.translation.x < -BORDER_DISTANCE {
            velocity.0.x = velocity.0.x.copysign(1.0);
        }
        if transform.translation.y > BORDER_DISTANCE {
            velocity.0.y = velocity.0.y.copysign(-1.0);
        } else if transform.translation.y < -BORDER_DISTANCE {
            velocity.0.y = velocity.0.y.copysign(1.0);
        }
    });
}

fn limit_speed(
    mut q: Query<&mut Velocity>
) {
    q.par_iter_mut().for_each(|mut velocity| {
        velocity.0 = velocity.0.clamp_length_max(MAX_SPEED);
    });
}


fn apply_particle_velocities(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Velocity)>
) {
    q.par_iter_mut().for_each(|(mut transform, velocity)| {
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    });
}
