use bevy:: {
    prelude::*,
    sprite::Mesh2dHandle
};

use crate::collider::{BORDER_DISTANCE, BORDER_THICKNESS};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, generate_assets);
    }
}

#[derive(Resource)]
pub struct ProbeAssets {
    pub triangle: Mesh2dHandle,
    pub blue: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct ParticleAssets {
    pub circle: Mesh2dHandle,
    pub red: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>
}

#[derive(Resource)]
pub struct ForcefieldAssets {
    pub triangle: Mesh2dHandle,
    pub green: Handle<ColorMaterial>
}

#[derive(Resource)]
pub struct ColliderAssets {
    pub h_rectangle: Mesh2dHandle,
    pub v_rectangle: Mesh2dHandle,
    pub white: Handle<ColorMaterial>
}

fn generate_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(
        ProbeAssets {
            triangle: Mesh2dHandle(meshes.add(
                                Triangle2d::new(
                                    Vec2::X * 250.0,
                                    Vec2::new(-100.0, -100.0),
                                    Vec2::new(-100.0, 100.0)
                                ))),
            blue: materials.add(Color::srgba(0.2, 0.2, 1.0, 0.25)),
        }
    );

    commands.insert_resource(
        ParticleAssets {
            circle: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            red: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            blue: materials.add(Color::srgb(0.0, 0.0, 1.0)),
        }
    );

    commands.insert_resource(
        ForcefieldAssets {
            triangle: Mesh2dHandle(meshes.add(
                                Triangle2d::new(
                                    Vec2::X * 250.0,
                                    Vec2::new(-250.0, -250.0),
                                    Vec2::new(-250.0, 250.0)
                                ))),
            green: materials.add(Color::srgba(0.0, 1.0, 0.0, 0.25)),
        }
    );

    commands.insert_resource(
        ColliderAssets {
            h_rectangle: Mesh2dHandle(meshes.add(Rectangle::new(2.0 * BORDER_DISTANCE, BORDER_THICKNESS))),
            v_rectangle: Mesh2dHandle(meshes.add(Rectangle::new(BORDER_THICKNESS, 2.0 * BORDER_DISTANCE))),
            white: materials.add(Color::srgb(1.0, 1.0, 1.0))
        }
    );
}
