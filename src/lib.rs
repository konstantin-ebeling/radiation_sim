use std::collections::HashMap;
use std::sync::atomic::Ordering;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::view::NoFrustumCulling,
};

use atomic_float::AtomicF32;

mod materials;
use materials::*;
mod render;
use render::*;
mod ui;
use ui::*;

pub struct RadiationSim;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default, Component, Reflect)]
#[reflect(Component)]
struct Particle {
    pub particle_type: ParticleType,
    pub energy: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Reflect)]
pub enum ParticleType {
    Alpha,
    Electron,
    Proton,
    Neutron,
    #[default]
    Gamma,
}

#[derive(Debug, Clone, PartialEq, Component)]
struct ParticleSpawner {
    /// how often different particles are supposed to spawn.
    /// first f32 is num/frame and second f32 for keeping track between frames
    pub spawns: HashMap<ParticleType, (f32, f32)>,
}

pub type ParticleMaterial = HashMap<ParticleType, f32>;

#[derive(Debug, Clone, PartialEq, Component)]
struct AmbientMaterial {
    pub material: ParticleMaterial,
}

#[derive(Debug, Clone, PartialEq, Default, Component)]
struct Obstacle {
    pub material: ParticleMaterial,
    pub absorbed_energy: f32,
}

#[derive(Debug, Clone, PartialEq, Default, Component, Reflect)]
#[reflect(Component)]
struct Velocity(Vec3);

struct Constants {
    light_speed: f32,
}
struct TimeData {
    time_step: f32,
    multi_step: usize,
}

impl Plugin for RadiationSim {
    fn build(&self, app: &mut App) {
        app.add_plugin(RadiationSimUI)
            .add_plugin(CustomMaterialPlugin)
            .register_type::<Particle>()
            .register_type::<ParticleType>()
            .register_type::<Velocity>()
            .insert_resource(Constants {
                light_speed: 299_792_458.0,
            })
            .insert_resource(TimeData {
                time_step: (10f32).powi(-12),
                multi_step: 16,
            })
            .add_startup_system(setup)
            .add_system(move_camera)
            .add_system(spawn_particles)
            .add_system(process_particles);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ------ World ------

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, 4.0),
        ..default()
    });

    // ------ Camera ------

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // ------ Particle Effects ------

    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Cube { size: 0.005 })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        InstanceMaterialData(vec![]),
        Visibility::default(),
        ComputedVisibility::default(),
        NoFrustumCulling,
    ));

    // spawner

    let mut spawns = HashMap::new();
    spawns.insert(ParticleType::Alpha, (200000000000.0, 0.0));
    spawns.insert(ParticleType::Electron, (200000000000.0, 0.0));
    spawns.insert(ParticleType::Gamma, (30_000_000_000_000.0, 0.0));

    commands
        .spawn()
        .insert(Name::new("Particle Spawner"))
        .insert(ParticleSpawner { spawns })
        .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
            0.0, 0.0, 0.0,
        )));

    // ambient material

    commands
        .spawn()
        .insert(Name::new("Ambient Material"))
        .insert(AmbientMaterial {
            material: air_material(),
        });

    // obstacle

    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let grey_material = materials.add(Color::rgb(0.4, 0.4, 0.4).into());

    commands
        .spawn()
        .insert(Name::new("Obstacle 1"))
        .insert_bundle(PbrBundle {
            material: grey_material.clone(),
            mesh: cube_mesh.clone(),
            transform: Transform::from_xyz(0.5, 0.5, 0.0).with_scale(Vec3::new(0.2, 1.0, 2.0)),
            ..Default::default()
        })
        .insert(Obstacle {
            material: dense_material(),
            ..Default::default()
        });

    commands
        .spawn()
        .insert(Name::new("Obstacle 2"))
        .insert_bundle(PbrBundle {
            material: grey_material.clone(),
            mesh: cube_mesh.clone(),
            transform: Transform::from_xyz(0.0, -0.5, 0.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
            ..Default::default()
        })
        .insert(Obstacle {
            material: dense_material(),
            ..Default::default()
        });
}

fn move_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut Camera)>,
) {
    for (mut transform, _) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        // forward/backwards
        for ev in scroll_evr.iter() {
            direction += transform.forward()
                * ev.y
                * if cfg!(target_arch = "wasm32") {
                    0.5
                } else {
                    15.0
                };
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += transform.forward() * 2.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction -= transform.forward() * 2.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction -= transform.right() * 2.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += transform.right() * 2.0;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            direction += transform.up() * 2.0;
        }
        if keyboard_input.pressed(KeyCode::LShift) {
            direction -= transform.up() * 2.0;
        }

        transform.translation += direction * time.delta_seconds() * 1.0;

        // look around
        if mouse_input.pressed(MouseButton::Right) {
            for ev in motion_evr.iter() {
                transform.rotate_y(ev.delta.x * -0.005);
                transform.rotate_local_x(ev.delta.y * -0.005);
            }
        }
    }
}

fn spawn_particles(
    time_data: Res<TimeData>,
    constants: Res<Constants>,
    mut query: Query<(&Transform, &mut ParticleSpawner)>,
    mut commands: Commands,
) {
    for (transform, mut spawner) in query.iter_mut() {
        for (particle, spawn_rate) in spawner.spawns.iter_mut() {
            // add number of particles that are supposed to be spawned per frame to inter frame counter
            spawn_rate.1 += spawn_rate.0 * time_data.time_step;

            // while inter frame counter above 1 spawn particles
            while spawn_rate.1 >= 1.0 {
                let velocity_direction = Vec3::new(
                    fastrand::f32() - 0.5,
                    fastrand::f32() - 0.5,
                    fastrand::f32() - 0.5,
                )
                .normalize();

                // spawn particle
                commands
                    .spawn()
                    .insert(Name::new("Particle"))
                    .insert_bundle(TransformBundle::from_transform(transform.clone()))
                    .insert(Particle {
                        energy: 1.0,
                        particle_type: particle.to_owned(),
                    })
                    .insert(Velocity(velocity_direction * constants.light_speed as f32))
                    .insert_bundle(VisibilityBundle::default());

                spawn_rate.1 -= 1.0;
            }
        }
    }
}

fn process_particles(
    par_commands: ParallelCommands,
    time_data: Res<TimeData>,
    ambient_query: Query<&AmbientMaterial>,

    mut query: Query<(Entity, &mut Transform, &mut Velocity, &mut Particle), Without<Obstacle>>,
    mut obstacle_query: Query<(&mut Obstacle, &Transform), Without<Particle>>,
) {
    let ambient_material = ambient_query.iter().next().unwrap();

    let obstacles = obstacle_query
        .iter_mut()
        .map(|q| (q, AtomicF32::new(0.0)))
        .collect::<Vec<_>>();

    query.par_for_each_mut(4096, |(entity, mut transform, velocity, mut particle)| {
        for _ in 0..time_data.multi_step {
            // move particle
            transform.translation += velocity.0 * time_data.time_step as f32;

            // collide particle
            let particle_type = particle.particle_type.clone();

            let mut hit_obstacle = false;
            for ((obstacle, obstacle_transform), absorbed_energy) in &obstacles {
                let pos = transform.translation;
                let obs_pos = obstacle_transform.translation;
                let obs_scale = obstacle_transform.scale;

                // check for hit
                if pos.x > obs_pos.x - obs_scale.x / 2.0
                    && pos.x < obs_pos.x + obs_scale.x / 2.0
                    && pos.y > obs_pos.y - obs_scale.y / 2.0
                    && pos.y < obs_pos.y + obs_scale.y / 2.0
                    && pos.z > obs_pos.z - obs_scale.z / 2.0
                    && pos.z < obs_pos.z + obs_scale.z / 2.0
                {
                    // apply material

                    let enery_transfer = obstacle.material.get(&particle_type).unwrap();

                    particle.energy -= enery_transfer;
                    absorbed_energy.fetch_add(*enery_transfer, Ordering::Relaxed);

                    hit_obstacle = true;
                }
            }

            if !hit_obstacle {
                particle.energy += ambient_material.material.get(&particle_type).unwrap();
            }

            if particle.energy < 0.0 {
                par_commands.command_scope(|mut commands| {
                    commands.entity(entity).despawn();
                });
                break;
            }
        }
    });

    for ((mut obstacle, _), absorbed_energy) in obstacles {
        obstacle.absorbed_energy += absorbed_energy.load(Ordering::Relaxed);
    }
}

pub fn run() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::INFO,
            filter: "spawn=trace,wgpu_core=warn,wgpu_hal=warn".to_string(),
        })
        .insert_resource(WindowDescriptor {
            fit_canvas_to_parent: true,
            // firefox hack
            #[cfg(target_arch = "wasm32")]
            height: 1080.0,
            resizable: true,
            title: "Radiation Simulation".to_owned(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 0.8)))
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(RadiationSim)
        .run();
}
