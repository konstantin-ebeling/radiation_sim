use bevy::prelude::*;

use crate::{
    presets, AmbientMaterial, AssetHandles, LinearSpawner, Object, ResetParticles, SubstanceData,
};

pub struct RadiationSimEnv;

impl Plugin for RadiationSimEnv {
    fn build(&self, app: &mut App) {
        app.add_state::<CurrentEnv>()
            .add_system(spawn_sandbox.in_schedule(OnEnter(CurrentEnv::Sandbox)))
            .add_system(despawn_sandbox.in_schedule(OnExit(CurrentEnv::Sandbox)))
            .add_system(spawn_experiment.in_schedule(OnEnter(CurrentEnv::Experiment)))
            .add_system(despawn_experiment.in_schedule(OnExit(CurrentEnv::Experiment)));
    }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum CurrentEnv {
    Sandbox,
    #[default]
    Experiment,
}

#[derive(Debug, Clone, Component)]
pub struct SandboxObject;

#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Human;
#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect(Component)]
pub struct HumanRoot;

fn spawn_sandbox(
    mut commands: Commands,
    asset_handles: ResMut<AssetHandles>,
    asset_server: Res<AssetServer>,
    substance_data: Res<SubstanceData>,
) {
    commands.spawn((
        AmbientMaterial {
            material: presets::air(&substance_data),
        },
        SandboxObject,
    ));

    // obstacles
    commands.spawn((
        Name::new("Wand"),
        PbrBundle {
            material: asset_handles.light_grey_material.as_ref().unwrap().clone(),
            mesh: asset_handles.cube_mesh.as_ref().unwrap().clone(),
            transform: Transform::from_xyz(0.5, 0.5, 0.0).with_scale(Vec3::new(0.01, 2.0, 2.0)),
            ..Default::default()
        },
        Object {
            material: presets::pb208(&substance_data),
            ..Default::default()
        },
        SandboxObject,
    ));

    commands.spawn((
        Name::new("Boden"),
        PbrBundle {
            material: asset_handles.grey_material.as_ref().unwrap().clone(),
            mesh: asset_handles.cube_mesh.as_ref().unwrap().clone(),
            transform: Transform::from_xyz(0.0, -0.5, 0.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
            ..Default::default()
        },
        Object {
            material: presets::pb208(&substance_data),
            ..Default::default()
        },
        SandboxObject,
    ));

    // spawner
    commands.spawn((
        Name::new("Strahlenquelle"),
        PbrBundle {
            material: asset_handles.light_grey_material.as_ref().unwrap().clone(),
            mesh: asset_handles.cube_mesh.as_ref().unwrap().clone(),
            transform: Transform::from_xyz(0.0, 0.1, 0.0).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..Default::default()
        },
        Object {
            material: presets::pu239(&substance_data),
            ..Default::default()
        },
        SandboxObject,
    ));

    spawn_human(&mut commands, &asset_server, &substance_data);
}

fn spawn_human(
    commands: &mut Commands,
    asset_server: &AssetServer,
    substance_data: &SubstanceData,
) {
    commands
        .spawn((
            SceneBundle {
                scene: asset_server.load("human_model/human.glb#Scene0"),
                transform: Transform::from_xyz(2.0, 0.0, 0.0),
                ..default()
            },
            Human,
            HumanRoot,
            SandboxObject,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Main Body"),
                TransformBundle {
                    local: Transform::from_xyz(0.0, 0.9, 0.0).with_scale(Vec3::new(0.27, 1.8, 0.2)),
                    ..Default::default()
                },
                Object {
                    material: presets::water(substance_data),
                    ..Default::default()
                },
                Human,
                SandboxObject,
            ));

            parent.spawn((
                Name::new("Arms"),
                TransformBundle {
                    local: Transform::from_xyz(0.0, 1.37, 0.0).with_scale(Vec3::new(1.7, 0.1, 0.1)),
                    ..Default::default()
                },
                Object {
                    material: presets::water(substance_data),
                    ..Default::default()
                },
                Human,
                SandboxObject,
            ));
        });
}

fn despawn_sandbox(
    mut commands: Commands,
    query: Query<Entity, With<SandboxObject>>,
    mut reset_event: EventWriter<ResetParticles>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    reset_event.send_default();
}

#[derive(Debug, Clone, Component)]
pub struct ExperimentObject;
#[derive(Debug, Clone, Component)]
pub struct ExperimentTarget;

fn spawn_experiment(
    mut commands: Commands,
    asset_handles: ResMut<AssetHandles>,
    substance_data: Res<SubstanceData>,
) {
    commands.spawn((
        AmbientMaterial {
            material: presets::vacuum(),
        },
        ExperimentObject,
    ));

    commands.spawn((
        Name::new("Boden"),
        PbrBundle {
            material: asset_handles
                .checkerboard_material
                .as_ref()
                .unwrap()
                .clone(),
            mesh: asset_handles.cube_mesh.as_ref().unwrap().clone(),
            transform: Transform::from_xyz(0.0, -0.5, 0.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
            ..Default::default()
        },
        Object {
            material: presets::pb208(&substance_data),
            ..Default::default()
        },
        ExperimentObject,
    ));

    commands.spawn((
        Name::new("Test"),
        PbrBundle {
            material: asset_handles.light_grey_material.as_ref().unwrap().clone(),
            mesh: asset_handles.cube_mesh.as_ref().unwrap().clone(),
            transform: Transform::from_xyz(0.06, 0.05, 0.0).with_scale(Vec3::new(0.001, 0.1, 0.1)),
            ..Default::default()
        },
        Object {
            material: presets::pb210(&substance_data),
            ..Default::default()
        },
        ExperimentObject,
        ExperimentTarget,
    ));

    commands.spawn((
        Name::new("Linear Quelle"),
        PbrBundle {
            material: asset_handles.light_grey_material.as_ref().unwrap().clone(),
            mesh: asset_handles.cube_mesh.as_ref().unwrap().clone(),
            transform: Transform::from_xyz(-0.06, 0.05, 0.0).with_scale(Vec3::new(0.01, 0.1, 0.1)),
            ..Default::default()
        },
        LinearSpawner {
            alpha_rate: 10_000_000_000.0,
            beta_rate: 100_000_000_000.0,
            gamma_rate: 100_000_000_000.0,
            particle_energy: 100_000.0,
        },
        ExperimentObject,
    ));

    commands.spawn((
        Name::new("Stop"),
        PbrBundle {
            material: asset_handles.light_grey_material.as_ref().unwrap().clone(),
            mesh: asset_handles.cube_mesh.as_ref().unwrap().clone(),
            transform: Transform::from_xyz(2.0, 0.5, 0.0).with_scale(Vec3::splat(1.0)),
            ..Default::default()
        },
        Object {
            material: presets::pb210(&substance_data),
            ..Default::default()
        },
        ExperimentObject,
    ));
}

fn despawn_experiment(
    mut commands: Commands,
    query: Query<Entity, With<ExperimentObject>>,
    mut reset_event: EventWriter<ResetParticles>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    reset_event.send_default();
}
