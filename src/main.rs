use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

mod asset;
mod particle;
mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanCamPlugin::default())
        .add_plugins(asset::AssetPlugin)
        .add_plugins(particle::ParticlePlugin)
        .add_plugins(physics::PhysicsPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                viewport_origin: Vec2::ZERO,
                near: -1000.0,
                ..default()
            },
            ..default()
		},
        PanCam::default()
    ));
}
