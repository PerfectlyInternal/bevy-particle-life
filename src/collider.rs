use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    math::bounding::Aabb2d,
};

use crate:: {
    particle::*,
    asset::*,
};

pub struct ColliderPlugin {
    pub parallel: bool
}

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_colliders);
        if self.parallel {
            app.add_systems(Update, handle_collisions_parallel);
        } else {
            app.add_systems(Update, handle_collisions_single_threaded);
        }
    }
}

pub const BORDER_DISTANCE: f32 = 5000.0;
pub const BORDER_THICKNESS: f32 = 100.0;

#[derive(Component)]
struct Collider {
    aabb: Aabb2d
}

fn spawn_colliders(
    mut commands: Commands,
    assets: Res<ColliderAssets>,
) {
    commands.spawn((
        Collider {
            aabb: Aabb2d {
                min: Vec2::new(-BORDER_DISTANCE, -BORDER_DISTANCE - BORDER_THICKNESS),
                max: Vec2::new(BORDER_DISTANCE, -BORDER_DISTANCE)
            }
        },
        MaterialMesh2dBundle {
            mesh: assets.h_rectangle.clone(),
            material: assets.white.clone(),
            transform: Transform::from_xyz(0.0, -BORDER_DISTANCE - BORDER_THICKNESS/2.0, 0.0), 
            ..default()
        }
    ));
    
    commands.spawn((
        Collider {
            aabb: Aabb2d {
                min: Vec2::new(-BORDER_DISTANCE, BORDER_DISTANCE),
                max: Vec2::new(BORDER_DISTANCE, BORDER_DISTANCE + BORDER_THICKNESS)
            }
        },
        MaterialMesh2dBundle {
            mesh: assets.h_rectangle.clone(),
            material: assets.white.clone(),
            transform: Transform::from_xyz(0.0, BORDER_DISTANCE + BORDER_THICKNESS/2.0, 0.0), 
            ..default()
        }
    ));

    commands.spawn((
        Collider {
            aabb: Aabb2d {
                min: Vec2::new(-BORDER_DISTANCE, -BORDER_DISTANCE),
                max: Vec2::new(BORDER_DISTANCE, -BORDER_DISTANCE - BORDER_THICKNESS)
            }
        },
        MaterialMesh2dBundle {
            mesh: assets.v_rectangle.clone(),
            material: assets.white.clone(),
            transform: Transform::from_xyz(-BORDER_DISTANCE - BORDER_THICKNESS/2.0, 0.0, 0.0),
            ..default()
        }
    ));
 
    commands.spawn((
        Collider {
            aabb: Aabb2d {
                min: Vec2::new(-BORDER_DISTANCE, BORDER_DISTANCE),
                max: Vec2::new(BORDER_DISTANCE, BORDER_DISTANCE + BORDER_THICKNESS)
            }
        },
        MaterialMesh2dBundle {
            mesh: assets.v_rectangle.clone(),
            material: assets.white.clone(),
            transform: Transform::from_xyz(BORDER_DISTANCE + BORDER_THICKNESS/2.0, 0.0, 0.0), 
            ..default()
        }
    ));
}



fn handle_collisions_parallel(
    
) {

}

fn handle_collisions_single_threaded(

) {

}

fn calculate_collision_velocity(
    aabb: Aabb2d,
    position: Vec2,
    velocity: Vec2
) -> Vec2 {
    Vec2::ZERO
}
