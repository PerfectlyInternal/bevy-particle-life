use bevy::{
    prelude::*,
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
};
use bevy_pancam::{PanCam, PanCamPlugin};

mod asset;
mod particle;
mod physics;

fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins,
                FpsOverlayPlugin {
                    config: FpsOverlayConfig {
                        text_config: TextStyle {
                            font_size: 50.0,
                            color: Color::srgb(0.0, 1.0, 0.0),
                            font: default(),
                        }
                    }
                },
                PanCamPlugin::default(),
                asset::AssetPlugin,
                particle::ParticlePlugin,
                physics::PhysicsPlugin
        ))
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
