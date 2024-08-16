use bevy:: {
    prelude::*,
    sprite::Mesh2dHandle
};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, generate_assets);
    }
}

#[derive(Resource)]
pub struct ParticleAssets {
    pub circle: Mesh2dHandle,
    pub red: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>
}

fn generate_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(
        ParticleAssets {
            circle: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
            red: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            blue: materials.add(Color::srgb(0.0, 0.0, 1.0)),
        }
    );
}
