use std::collections::BTreeMap;
use std::sync::atomic::Ordering;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::view::NoFrustumCulling,
};

use atomic_float::AtomicF32;

mod data_reading;
use data_reading::*;
mod material;
use material::*;
mod render;
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

#[derive(Debug, Clone, Component)]
struct AmbientMaterial {
    pub material: MaterialData,
}

#[derive(Debug, Clone, Default, Component)]
struct Object {
    pub material: MaterialData,
    pub absorbed_energy: f32,
}

#[derive(Debug, Clone, PartialEq, Default, Component, Reflect)]
#[reflect(Component)]
struct Velocity(Vec3);

#[derive(Debug)]
pub struct Constants {
    pub light_speed: f32,
    pub avogadro_constant: f32,
    pub ev_conversion: f32,
    pub electron_mass: f32,
    pub aplha_mass: f32,

    pub elements: BTreeMap<usize, Element>,
    pub compounds: BTreeMap<String, Compound>,
    pub radiators: Vec<SubstanceIdentifier>,
    pub absorbers: Vec<SubstanceIdentifier>,
}

#[derive(Debug)]
struct TimeData {
    time_step_move: f32,
    time_step_calc: f32,
    multi_step: usize,
}

struct InterfaceState {
    advanced: bool,
}

impl Plugin for RadiationSim {
    fn build(&self, app: &mut App) {
        app.add_plugin(RadiationSimUI)
            .add_plugin(render::CustomMaterialPlugin)
            .register_type::<Particle>()
            .register_type::<ParticleType>()
            .register_type::<Velocity>()
            .insert_resource(Constants {
                light_speed: 299_792_458.0,
                avogadro_constant: 6.02214076 * (10f32).powi(23),
                ev_conversion: 1.602 * (10f32).powi(-19),
                electron_mass: 9.1093837015 * (10f32).powi(-31),
                aplha_mass: 6.6446573357 * (10f32).powi(-27),

                elements: BTreeMap::new(),
                compounds: BTreeMap::new(),
                radiators: Vec::new(),
                absorbers: Vec::new(),
            })
            .insert_resource(TimeData {
                time_step_move: (10f32).powi(-12),
                time_step_calc: (10f32).powi(-11),
                multi_step: 16,
            })
            .insert_resource(InterfaceState {
                // in debug builds show advanced default
                advanced: cfg!(debug_assertions),
            })
            .insert_resource(AmbientLight {
                brightness: 0.1,
                color: Color::rgb(1.0, 1.0, 1.0),
            })
            .add_startup_system(setup)
            .add_startup_system(read_data)
            .add_system(move_camera)
            .add_system(spawn_particles)
            .add_system(process_particles);
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ------ World ------

    let mut light_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    light_transform.rotate_y(std::f32::consts::PI / -5.0);
    light_transform.rotate_x(std::f32::consts::PI / -6.0);
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            illuminance: 5000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: light_transform,
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
        render::InstanceMaterialData(vec![]),
        Visibility::default(),
        ComputedVisibility::default(),
        NoFrustumCulling,
    ));

    // ambient material

    commands
        .spawn()
        .insert(Name::new("Ambient Material"))
        .insert(AmbientMaterial {
            material: presets::air(),
        });

    // obstacle

    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let grey_material = materials.add(Color::rgb(0.6, 0.6, 0.6).into());
    let light_grey_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());

    commands
        .spawn()
        .insert(Name::new("Obstacle 1"))
        .insert_bundle(PbrBundle {
            material: light_grey_material.clone(),
            mesh: cube_mesh.clone(),
            transform: Transform::from_xyz(0.5, 0.5, 0.0).with_scale(Vec3::new(0.2, 1.0, 2.0)),
            ..Default::default()
        })
        .insert(Object {
            material: presets::lead(),
            ..Default::default()
        });

    commands
        .spawn()
        .insert(Name::new("Floor"))
        .insert_bundle(PbrBundle {
            material: grey_material.clone(),
            mesh: cube_mesh.clone(),
            transform: Transform::from_xyz(0.0, -0.5, 0.0).with_scale(Vec3::new(100.0, 1.0, 100.0)),
            ..Default::default()
        })
        .insert(Object {
            material: presets::lead(),
            ..Default::default()
        });

    // human
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("human_model/scene.gltf#Scene0"),
        transform: Transform::from_xyz(1.5, 0.0, 0.0).with_scale(Vec3::splat(0.3)),
        ..default()
    });

    // spawner (plutonium block)

    commands
        .spawn()
        .insert(Name::new("Spawner"))
        .insert_bundle(PbrBundle {
            material: light_grey_material.clone(),
            mesh: cube_mesh.clone(),
            transform: Transform::from_xyz(0.0, 0.1, 0.0).with_scale(Vec3::new(0.2, 0.2, 0.2)),
            ..Default::default()
        })
        .insert(Object {
            material: presets::plutonium(),
            ..Default::default()
        });
}

fn read_data(mut constants: ResMut<Constants>) {
    // elements
    let element_data = element::get_elemnts(&constants);
    let mut element_hashmap = BTreeMap::new();
    for element in element_data {
        element_hashmap.insert(element.z, element);
    }
    constants.elements = element_hashmap;

    let mut radiators = Vec::new();
    let mut absorbers = Vec::new();
    for (z, element) in &constants.elements {
        for (n, isotope) in &element.isotopes {
            if isotope.is_usable {
                radiators.push(SubstanceIdentifier::Element(z.to_owned(), n.to_owned()));
            }
        }
        if element.is_absorber {
            let mut isotopes_sorted = element.isotopes.clone().into_values().collect::<Vec<_>>();
            isotopes_sorted.sort_by_key(|i| i.abundance);
            absorbers.push(SubstanceIdentifier::Element(
                z.to_owned(),
                isotopes_sorted.last().unwrap().n,
            ));
        }
    }
    constants.radiators = radiators;
    constants.absorbers = absorbers;

    // compounds
    let compound_data = compound::get_compounds();
    let mut compound_hashmap = BTreeMap::new();
    for compound in compound_data {
        compound_hashmap.insert(compound.name.to_owned(), compound);
    }
    constants.compounds = compound_hashmap;

    let mut absorbers = Vec::new();
    for (name, compound) in &constants.compounds {
        if compound.is_absorber {
            absorbers.push(SubstanceIdentifier::Compound(name.to_owned()));
        }
    }
    constants.absorbers.extend(absorbers);

    // nice logs
    for e in &constants.radiators {
        match &e {
            SubstanceIdentifier::Element(ref z, ref n) => {
                let element = &constants.elements[z];
                let isotope = &element.isotopes[&n];
                log::info!(
                    "{} {:?}: {:?} eV, {:?} ev, {} Bq/kg",
                    element.symbol,
                    z + n,
                    isotope.decays[0].decay_energy,
                    isotope.decays[0].gamma_energy,
                    isotope.activity.unwrap()
                );
            }
            SubstanceIdentifier::Compound(ref name) => {
                log::info!("{}", &name);
            }
        }
    }

    for e in &constants.absorbers {
        match &e {
            SubstanceIdentifier::Element(ref z, _) => {
                let element = &constants.elements[z];
                log::info!("{} Absorber", element.symbol);
            }
            SubstanceIdentifier::Compound(ref name) => {
                log::info!("{} Absorber", &name);
            }
        }
    }
}

fn move_camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut Camera)>,
    interface_state: Res<InterfaceState>,
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

        if interface_state.advanced {
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

            // look around
            if mouse_input.pressed(MouseButton::Right) {
                for ev in motion_evr.iter() {
                    transform.rotate_y(ev.delta.x * -0.005);
                    transform.rotate_local_x(ev.delta.y * -0.005);
                }
            }
        } else {
            // orbit around 0,0,0
            if mouse_input.pressed(MouseButton::Left) {
                for ev in motion_evr.iter() {
                    transform.rotate_around(
                        Vec3::ZERO,
                        Quat::from_euler(EulerRot::YXZ, ev.delta.x * -0.005, 0.0, 0.0),
                    );
                    let right = transform.local_x();
                    transform.rotate_around(
                        Vec3::ZERO,
                        Quat::from_axis_angle(right, ev.delta.y * -0.005),
                    );
                }

                transform.look_at(Vec3::ZERO, Vec3::Y);
            }
        }

        transform.translation += direction * time.delta_seconds() * 1.0;
    }
}

fn spawn_particles(
    time_data: Res<TimeData>,
    constants: Res<Constants>,
    query: Query<(&Transform, &Object)>,
    mut commands: Commands,
) {
    for (transform, object) in query.iter() {
        let substance = object.material.pick_substance(&constants);

        match &substance {
            Substance::Element((element, n)) => {
                if element.isotopes[n].is_usable {
                    let volume = transform.scale.x * transform.scale.y * transform.scale.z;
                    let weight = volume * element.density;
                    let estimated_decays =
                        element.isotopes[n].activity.unwrap() * weight * time_data.time_step_calc;

                    let decays = estimated_decays.floor() as usize
                        + if (estimated_decays - estimated_decays.floor()) > fastrand::f32() {
                            1
                        } else {
                            0
                        };

                    for _ in 0..decays {
                        let velocity_direction = Vec3::new(
                            fastrand::f32() - 0.5,
                            fastrand::f32() - 0.5,
                            fastrand::f32() - 0.5,
                        )
                        .normalize();

                        let pos_offset = Vec3::new(
                            transform.scale.x * (fastrand::f32() - 0.5),
                            transform.scale.y * (fastrand::f32() - 0.5),
                            transform.scale.z * (fastrand::f32() - 0.5),
                        );

                        let decay = &element.isotopes[n].decays[0];

                        let particle_type = match decay.decay_type {
                            element::DecayType::Alpha => ParticleType::Alpha,
                            element::DecayType::BetaElectronCapture => ParticleType::Electron,
                            element::DecayType::BetaMinus => ParticleType::Electron,
                            element::DecayType::BetaPlus => ParticleType::Electron,
                            _ => panic!("incorrect decay type"),
                        };

                        // spawn particle
                        commands
                            .spawn()
                            .insert_bundle(TransformBundle::from_transform(
                                Transform::from_translation(
                                    transform.clone().translation + pos_offset,
                                ),
                            ))
                            .insert(Particle {
                                // these have energy as velocity
                                energy: 0.0,
                                particle_type: particle_type.clone(),
                            })
                            .insert(Velocity(
                                velocity_direction
                                    * energy_to_velocity(
                                        &decay.decay_energy,
                                        &particle_type,
                                        &constants,
                                    ),
                            ))
                            .insert_bundle(VisibilityBundle::default());

                        // spawn gamma ray
                        if let Some(gamma_energy) = decay.gamma_energy {
                            commands
                                .spawn()
                                .insert_bundle(TransformBundle::from_transform(
                                    Transform::from_translation(
                                        transform.clone().translation + pos_offset,
                                    ),
                                ))
                                .insert(Particle {
                                    energy: gamma_energy,
                                    particle_type: ParticleType::Gamma,
                                })
                                .insert(Velocity(
                                    velocity_direction * (constants.light_speed as f32),
                                ))
                                .insert_bundle(VisibilityBundle::default());
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn process_particles(
    time_data: Res<TimeData>,
    constants: Res<Constants>,

    ambient_query: Query<&AmbientMaterial>,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &mut Particle), Without<Object>>,
    mut object_query: Query<(&mut Object, &Transform), Without<Particle>>,

    par_commands: ParallelCommands,
) {
    let ambient_material = ambient_query.iter().next().unwrap();

    let objects = object_query
        .iter_mut()
        .map(|q| (q, AtomicF32::new(0.0)))
        .collect::<Vec<_>>();

    query.par_for_each_mut(
        4096,
        |(entity, mut transform, mut velocity, mut particle)| {
            for _ in 0..time_data.multi_step {
                // move particle
                transform.translation += velocity.0 * time_data.time_step_move as f32;

                // collide particle
                let mut hit_obstacle = false;
                for ((object, obstacle_transform), absorbed_energy) in &objects {
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
                        let energy = match particle.particle_type {
                            ParticleType::Gamma => particle.energy,
                            _ => velocity_to_energy(
                                &velocity.0.clone().length(),
                                &particle.particle_type,
                                &constants,
                            ),
                        };

                        let substance = object.material.pick_substance(&constants);

                        // let stopping_power = pick_stopping_power(
                        //     &substance.stopping_power(&particle.particle_type),
                        //     energy,
                        // );

                        let is_spawner = match &substance {
                            Substance::Element((element, n)) => element.isotopes[n].is_usable,
                            _ => false,
                        };

                        if !is_spawner {
                            match &particle.particle_type {
                                ParticleType::Gamma => {
                                    particle.energy -= 1_000.0;
                                }
                                _ => {
                                    let mut vel = velocity.0.length();
                                    vel -= 2_000.0;
                                    velocity.0 = velocity.0.normalize() * vel;
                                }
                            }

                            // particle.energy -= enery_transfer;
                            // absorbed_energy.fetch_add(*enery_transfer, Ordering::Relaxed);

                            hit_obstacle = true;
                        }
                    }
                }

                if !hit_obstacle {
                    match &particle.particle_type {
                        ParticleType::Gamma => {
                            particle.energy -= 10.0;
                        }
                        _ => {
                            let mut vel = velocity.0.length();
                            vel -= 2_000.0;
                            velocity.0 = velocity.0.normalize() * vel;
                        }
                    }
                }

                if particle.energy < 0.0 || velocity.0.length() < 20_000.0 {
                    par_commands.command_scope(|mut commands| {
                        commands.entity(entity).despawn();
                    });
                    break;
                }
            }
        },
    );

    for ((mut obstacle, _), absorbed_energy) in objects {
        obstacle.absorbed_energy += absorbed_energy.load(Ordering::Relaxed);
    }
}

fn pick_stopping_power(stopping_powers: &StoppingPower, energy: f32) -> f32 {
    for (stop_energy, stopping_power) in stopping_powers {
        if *stop_energy < energy {
            return *stopping_power;
        }
    }
    return stopping_powers.last().unwrap().1;
}

fn energy_to_velocity(energy: &f32, particle_type: &ParticleType, constants: &Constants) -> f32 {
    // TODO: account for relavistiuc movement (thanks Einstein...)
    (2.0 * energy * constants.ev_conversion
        / match particle_type {
            ParticleType::Alpha => constants.aplha_mass,
            _ => constants.electron_mass,
        })
    .sqrt()
}

fn velocity_to_energy(velocity: &f32, particle_type: &ParticleType, constants: &Constants) -> f32 {
    // TODO: account for relavistiuc movement (thanks Einstein...)
    ((match particle_type {
        ParticleType::Alpha => constants.aplha_mass,
        _ => constants.electron_mass,
    } * velocity.powi(2))
        / 2.0)
        / constants.ev_conversion
}

pub fn run() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::INFO,
            filter: "spawn=trace,wgpu_core=warn,wgpu_hal=warn".to_string(),
        })
        .insert_resource(WindowDescriptor {
            fit_canvas_to_parent: true,
            resizable: true,
            canvas: Some("#maincanvas".to_owned()),
            title: "Radiation Simulation".to_owned(),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(RadiationSim)
        .run();
}
