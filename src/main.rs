use bevy::prelude::*;

mod asset;
mod ui;
mod particle;
mod physics;

fn main() {
    App::new()
        .add_plugins((
                DefaultPlugins,
                ui::UIPlugin,
                asset::AssetPlugin,
                particle::ParticlePlugin,
                physics::PhysicsPlugin { parallel: true }
        ))
        .run();
}
