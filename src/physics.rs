use bevy::prelude::*;

use crate:: {
    particle::*,
    forcefield::ForcefieldPlugin,
    collider::ColliderPlugin,
};

pub struct PhysicsPlugin {
    pub parallel: bool,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ForcefieldPlugin { parallel: self.parallel });
        app.add_plugins(ColliderPlugin { parallel: self.parallel });
        app.insert_resource(TotalKineticEnergy(0.0));
        app.add_systems(Update, update_kinetic_energy);
        if self.parallel {
            app.add_systems(Update, apply_particle_forces_parallel);
        } else {
            app.add_systems(Update, apply_particle_forces_combination);
        }
        app.add_systems(Update, (border_interaction, limit_speed));
        app.add_systems(PostUpdate, apply_particle_velocities);
    }
}

const K: f32 = 1000000.0;
const BORDER_DISTANCE: f32 = 5000.0;
const MAX_SPEED: f32 = 1000.0;
const MAX_INTERACTION_DISTANCE: f32 = 500.0;
const DAMPING_COEFF: f32 = 0.999;

#[derive(Resource)]
pub struct TotalKineticEnergy(pub f32);

fn update_kinetic_energy(
    mut kenergy: ResMut<TotalKineticEnergy>,
    q: Query<&Velocity>,
) {
    kenergy.0 = 0.0;
    for velocity in q.iter() {
        kenergy.0 += velocity.0.length().powf(2.0);
    }
}

fn apply_particle_forces_combination(
    time: Res<Time>,
    mut q: Query<(&mut Velocity, &Charge, &Transform)>,
) {
    let mut combinations = q.iter_combinations_mut();
    while let Some(
        [(mut velocity_a, charge_a, transform_a),
         (mut velocity_b, charge_b, transform_b)])
        = combinations.fetch_next()
    {
        let force = calculate_particle_force(
                transform_a.translation,
                transform_b.translation,
                charge_a.0,
                charge_b.0
        );
        velocity_a.0 += (force * time.delta_seconds()).xy();
        velocity_b.0 -= (force * time.delta_seconds()).xy();
    }
}

fn apply_particle_forces_parallel(
    time: Res<Time>,
    mut q: Query<(&mut Velocity, &Charge, &Transform)>,
    q2: Query<(&Charge, &Transform)>
) {
    q.par_iter_mut().for_each(|(mut velocity_a, charge_a, transform_a)| {
        for (charge_b, transform_b) in q2.iter() {
            if transform_a == transform_b {
                continue;
            }
            let force = calculate_particle_force(
                transform_a.translation,
                transform_b.translation,
                charge_a.0,
                charge_b.0
            );
            velocity_a.0 += (force * time.delta_seconds()).xy();
        }
    });
}

fn calculate_particle_force(
    pos_a: Vec3,
    pos_b: Vec3,
    charge_a: f32,
    charge_b: f32
) -> Vec3 {
    let delta = pos_a - pos_b;
    let direction = delta.normalize_or_zero();
    let distance = delta.length();
    if distance > MAX_INTERACTION_DISTANCE { return Vec3::ZERO; }
    let force = K * ((charge_a * charge_b) / f32::powf(distance, 2.0));
    force * direction
}

fn border_interaction(
    time: Res<Time>,
    mut q: Query<(&mut Velocity, &Transform)>
) {
    q.par_iter_mut().for_each(|(mut velocity, transform)| {
        if transform.translation.x > BORDER_DISTANCE {
            velocity.0.x = velocity.0.x.copysign(-1.0) - 0.1 * time.delta_seconds();
        } else if transform.translation.x < -BORDER_DISTANCE {
            velocity.0.x = velocity.0.x.copysign(1.0) + 0.1 * time.delta_seconds();
        }
        if transform.translation.y > BORDER_DISTANCE {
            velocity.0.y = velocity.0.y.copysign(-1.0) - 0.1 * time.delta_seconds();
        } else if transform.translation.y < -BORDER_DISTANCE {
            velocity.0.y = velocity.0.y.copysign(1.0) + 0.1 * time.delta_seconds();
        }
    });
}

fn limit_speed(
    mut q: Query<&mut Velocity>
) {
    q.par_iter_mut().for_each(|mut velocity| {
        velocity.0 = velocity.0.clamp_length_max(MAX_SPEED) * DAMPING_COEFF;
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
