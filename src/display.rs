use bevy:: {
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use std::f32::consts::PI;

use crate:: {
    asset::ProbeAssets,
    particle::*,
};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_probes);
        app.add_systems(Update, (
                update_density_probes_count,
                update_velocity_probes_count)
            );
        app.add_systems(Update, update_velocity_probes_visuals);
    }
}

#[derive(Component)]
struct DensityProbe {
    radius: f32,
    count: u32
}

#[derive(Component)]
struct VelocityProbe {
    radius: f32,
    velocity: Vec2
}

fn spawn_probes(
    mut commands: Commands,
    assets: Res<ProbeAssets>,
) {
    for x in -8..8 {
        for y in -8..8 {
            commands.spawn((
                VelocityProbe {
                    radius: 250.0,
                    velocity: Vec2::ZERO
                },
                MaterialMesh2dBundle {
                    mesh: assets.triangle.clone(),
                    material: assets.blue.clone(),
                    transform: Transform::from_xyz(
                        500.0 * x as f32,
                        500.0 * y as f32,
                        0.0),
                    ..default()
                }
            ));
        }
    }
}

fn update_density_probes_count(
    mut q_probes: Query<(&mut DensityProbe, &Transform)>,
    q_particles: Query<&Transform, With<Particle>>,
) {
    q_probes.par_iter_mut().for_each(|(mut probe, transform)| {
        for particle in q_particles.iter() {
            if (transform.translation - particle.translation).length() < probe.radius {
                probe.count += 1;
            }
        }
    });
}

fn update_velocity_probes_count(
    mut q_probes: Query<(&mut VelocityProbe, &Transform)>,
    q_particles: Query<(&Transform, &Velocity), With<Particle>>,
) {
    q_probes.par_iter_mut().for_each(|(mut probe, transform)| {
        probe.velocity = Vec2::ZERO;
        let mut count: u32 = 0;
        for (particle, velocity) in q_particles.iter() {
            if (transform.translation - particle.translation).length() < probe.radius {
                probe.velocity += velocity.0;
                count += 1;
            }
        }
        probe.velocity /= count as f32;
    });
}

fn update_velocity_probes_visuals(
    mut q: Query<(&VelocityProbe, &mut Transform)>,
) {
    for (probe, mut transform) in q.iter_mut() {
        let angle = f32::atan(probe.velocity.y / probe.velocity.x);
        if probe.velocity.x < 0.0 {
            transform.rotation = Quat::from_rotation_z(angle + PI);
        } else {
            transform.rotation = Quat::from_rotation_z(angle);
        }
        transform.scale.x = probe.velocity.length() / 1000.0;
        transform.scale.y = probe.velocity.length() / 1000.0;
    }
}
