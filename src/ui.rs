use bevy::{
    prelude::*,
    dev_tools::fps_overlay::FpsOverlayPlugin,
};

use bevy_pancam:: {
    PanCam,
    PanCamPlugin
};

use crate:: {
    particle::ParticleCounter,
    physics::TotalKineticEnergy,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin);
        app.add_plugins(FpsOverlayPlugin::default());
        app.add_systems(Startup, (setup_ui, setup_camera));
        app.add_systems(Update, (update_counter, update_kinetic_energy));
    }
}

#[derive(Component)]
struct CounterText;

#[derive(Component)]
struct KineticEnergyText;

fn setup_ui(
    mut commands: Commands
) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Particles: ",
                TextStyle {
                    font_size: 40.0,
                    ..default()
                }
            ),
            TextSection::from_style(
                TextStyle {
                    font_size: 40.0,
                    ..default()
                }
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        CounterText
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Kinetic Energy: ",
                TextStyle {
                    font_size: 40.0,
                    ..default()
                }
            ),
            TextSection::from_style(
                TextStyle {
                    font_size: 40.0,
                    ..default()
                }
            ),
            TextSection::new(
                " MJ",
                TextStyle {
                    font_size: 40.0,
                    ..default()
                }
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            left: Val::Px(15.0),
            ..default()
        }),
        KineticEnergyText
    ));
}

fn update_counter(
    counter: Res<ParticleCounter>,
    mut q: Query<&mut Text, With<CounterText>>,
) {
    let mut text = q.single_mut();
    let count = counter.0;
    text.sections[1].value = format!("{count}");
}

fn update_kinetic_energy(
    energy: Res<TotalKineticEnergy>,
    mut q: Query<&mut Text, With<KineticEnergyText>>,
) {
    let mut text = q.single_mut();
    let value = energy.0 / 1_000_000.0;
    text.sections[1].value = format!("{value:.2}");
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
