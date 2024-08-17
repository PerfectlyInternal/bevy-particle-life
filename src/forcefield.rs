use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use crate:: {
    particle::*,
    asset::*,
};

pub struct ForcefieldPlugin {
    pub parallel: bool
}

impl Plugin for ForcefieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_forcefield);
        if self.parallel {
            app.add_systems(Update, apply_forcefields_single_thread);
        } else {
            app.add_systems(Update, apply_forcefields_parallel);
        }
    }
}

#[derive(Component)]
pub struct Forcefield {
    rect: Rect,
    force: Vec2,
}

fn spawn_forcefield(
    mut commands: Commands,
    assets: Res<ForcefieldAssets>,
) {
    commands.spawn((
        Forcefield {
            rect: Rect {
                min: Vec2 { x: -500.0, y: -500.0 },
                max: Vec2 { x: 500.0, y: 500.0 }
            },
            force: Vec2 { x: (500.0), y: (0.0) }
        },
        MaterialMesh2dBundle {
            mesh: assets.triangle.clone(),
            material: assets.green.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0), 
            ..default()
        }
    ));
}

fn apply_forcefields_single_thread(
    time: Res<Time>,
    q_forcefields: Query<&Forcefield>,
    mut q_particles: Query<(&mut Velocity, &Transform)>,
) {
    for forcefield in q_forcefields.iter() {
        for (mut velocity, transform) in q_particles.iter_mut() {
            if forcefield.rect.contains(transform.translation.xy()) {
                velocity.0 += forcefield.force * time.delta_seconds();
            }
        }
    }
}

fn apply_forcefields_parallel(
    time: Res<Time>,
    q_forcefields: Query<&Forcefield>,
    mut q_particles: Query<(&mut Velocity, &Transform)>,
) {
    q_particles.par_iter_mut().for_each(|(mut velocity, transform)| {
        for forcefield in q_forcefields.iter() {
            if forcefield.rect.contains(transform.translation.xy()) {
                velocity.0 += forcefield.force * time.delta_seconds();
            }
        }
    });
}
