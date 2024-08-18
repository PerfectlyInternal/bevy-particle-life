use bevy::prelude::*;

mod asset;
mod ui;
mod display;
mod particle;
mod physics;
mod emmiter;
mod forcefield;
mod collider;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            display::DisplayPlugin,
            ui::UIPlugin,
            asset::AssetPlugin,
            particle::ParticlePlugin,
            physics::PhysicsPlugin { parallel: true }
        ))
        .run();
}
