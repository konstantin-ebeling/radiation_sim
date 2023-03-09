use std::collections::BTreeMap;
use std::sync::{atomic::Ordering, Arc};

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::view::NoFrustumCulling,
};

use atomic_float::AtomicF32;

pub mod constants;
pub use constants::*;
mod data_reading;
use data_reading::*;
pub mod material;
use material::*;
mod render;
mod ui;
use ui::*;

pub struct RadiationSim;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Particle {
    pub particle_type: ParticleType,
    pub energy: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash, Reflect)]
pub enum ParticleType {
    Alpha,
    Electron,
    Proton,
    Neutron,
    #[default]
    Gamma,
}

#[derive(Debug, Clone, Component)]
pub struct AmbientMaterial {
    pub material: MaterialData,
}

#[derive(Debug, Clone, Default, Component)]
pub struct Object {
    pub material: MaterialData,
    pub absorbed_energy: f32,
}

#[derive(Debug, Clone, PartialEq, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Velocity(Vec3);

#[derive(Debug, Component)]
pub struct Human;
#[derive(Debug, Component)]
pub struct HumanRoot;

#[derive(Debug, Resource)]
pub struct SubstanceData {
    pub elements: BTreeMap<usize, Arc<Element>>,
    pub compounds: BTreeMap<String, Arc<Compound>>,
    pub radiators: Vec<Substance>,
    pub absorbers: Vec<Substance>,
}

#[derive(Debug, Resource)]
pub struct TimeData {
    time_step_move: f32,
    time_step_calc: f32,
    multi_step: usize,
    halted: bool,
    time_passed: f32,
}

#[derive(Debug, Resource)]
pub struct InterfaceState {
    advanced: bool,
    edit_objects: bool,
}

#[derive(Debug, Resource)]
pub struct AssetHandles {
    cube_mesh: Option<Handle<Mesh>>,
    grey_material: Option<Handle<StandardMaterial>>,
    light_grey_material: Option<Handle<StandardMaterial>>,
}

impl Plugin for RadiationSim {
    fn build(&self, app: &mut App) {
        app.add_plugin(RadiationSimUI)
            .add_plugin(render::CustomMaterialPlugin)
            .register_type::<Particle>()
            .register_type::<ParticleType>()
            .register_type::<Velocity>()
            .insert_resource(SubstanceData {
                elements: BTreeMap::new(),
                compounds: BTreeMap::new(),
                radiators: Vec::new(),
                absorbers: Vec::new(),
            })
            .insert_resource(TimeData {
                time_step_move: (10f32).powi(-12),
                time_step_calc: (10f32).powi(-11),
                multi_step: 16,
                halted: false,
                time_passed: 0.0,
            })
            .insert_resource(InterfaceState {
                // in debug builds show advanced default
                advanced: cfg!(debug_assertions),
                edit_objects: cfg!(debug_assertions),
            })
            .insert_resource(AssetHandles {
                cube_mesh: None,
                grey_material: None,
                light_grey_material: None,
            })
            .insert_resource(AmbientLight {
                brightness: 0.1,
                color: Color::rgb(1.0, 1.0, 1.0),
            })
            .add_startup_system(read_data)
            .add_startup_system(setup.after(read_data))
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
    mut asset_handles: ResMut<AssetHandles>,

    substance_data: Res<SubstanceData>,
) {
    // ------ World ------

    let mut light_transform = Transform::from_xyz(0.0, 0.0, 0.0);
    light_transform.rotate_y(std::f32::consts::PI / -5.0);
    light_transform.rotate_x(std::f32::consts::PI / -6.0);
    commands.spawn(DirectionalLightBundle {
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

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // ------ Particle Effects ------

    commands.spawn((
        meshes.add(Mesh::from(shape::Cube { size: 0.005 })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        render::InstanceMaterialData(vec![]),
        Visibility::default(),
        ComputedVisibility::default(),
        NoFrustumCulling,
    ));

    // ambient material

    commands.spawn(AmbientMaterial {
        material: presets::air(&substance_data),
    });

    // obstacles

    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    asset_handles.cube_mesh = Some(cube_mesh);
    let grey_material = materials.add(Color::rgb(0.6, 0.6, 0.6).into());
    asset_handles.grey_material = Some(grey_material);
    let light_grey_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    asset_handles.light_grey_material = Some(light_grey_material);

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
    ));

    // human
    commands
        .spawn((
            SceneBundle {
                scene: asset_server.load("human_model/human.glb#Scene0"),
                transform: Transform::from_xyz(2.0, 0.0, 0.0),
                ..default()
            },
            Human,
            HumanRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Main Body"),
                TransformBundle {
                    local: Transform::from_xyz(0.0, 0.9, 0.0).with_scale(Vec3::new(0.27, 1.8, 0.2)),
                    ..Default::default()
                },
                Object {
                    material: presets::water(&substance_data),
                    ..Default::default()
                },
                Human,
            ));

            parent.spawn((
                Name::new("Arms"),
                TransformBundle {
                    local: Transform::from_xyz(0.0, 1.37, 0.0).with_scale(Vec3::new(1.7, 0.1, 0.1)),
                    ..Default::default()
                },
                Object {
                    material: presets::water(&substance_data),
                    ..Default::default()
                },
                Human,
            ));
        });
}

fn read_data(mut substance_data: ResMut<SubstanceData>) {
    // elements
    let element_data = element::get_elements();
    let mut element_btree = BTreeMap::new();
    for element in element_data {
        element_btree.insert(element.z, element);
    }
    substance_data.elements = element_btree;

    let mut radiators = Vec::new();
    let mut absorbers = Vec::new();
    for (_, element) in &substance_data.elements {
        for (n, isotope) in &element.isotopes {
            if isotope.is_usable {
                radiators.push(Substance::Element(element.clone(), *n));
            }
        }
        if element.is_absorber {
            let mut isotopes_sorted = element.isotopes.clone().into_values().collect::<Vec<_>>();
            isotopes_sorted.sort_by_key(|i| i.abundance);
            absorbers.push(Substance::Element(
                element.clone(),
                isotopes_sorted.last().unwrap().n,
            ));
        }
    }
    substance_data.radiators = radiators;
    substance_data.absorbers = absorbers;

    // compounds
    let compound_data = compound::get_compounds();
    let mut compound_btree = BTreeMap::new();
    for compound in compound_data {
        compound_btree.insert(compound.name.to_owned(), compound);
    }
    substance_data.compounds = compound_btree;

    let mut absorbers = Vec::new();
    for (_, compound) in &substance_data.compounds {
        if compound.is_absorber {
            absorbers.push(Substance::Compound(compound.clone()));
        }
    }
    substance_data.absorbers.extend(absorbers);

    // nice logs
    for e in &substance_data.radiators {
        match &e {
            Substance::Element(element, n) => {
                let isotope = &element.isotopes[n];
                log::info!(
                    "{} {:?}: {:?} eV, {:?} ev, {} Bq/kg",
                    element.symbol,
                    element.z + n,
                    isotope.decays[0].decay_energy,
                    isotope.decays[0].gamma_energy,
                    isotope.activity.unwrap()
                );
            }
            Substance::Compound(compound) => {
                log::info!("{}", &compound.name);
            }
        }
    }

    for e in &substance_data.absorbers {
        match &e {
            Substance::Element(element, _) => {
                log::info!("{} Absorber", element.symbol);
            }
            Substance::Compound(compound) => {
                log::info!("{} Absorber", compound.name);
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
    mut time_data: ResMut<TimeData>,
    query: Query<(&Transform, &GlobalTransform, &Object)>,
    mut commands: Commands,
) {
    if time_data.halted {
        return;
    }

    time_data.time_passed = time_data.time_step_calc * time_data.multi_step as f32;

    for (transform, global_transform, object) in query.iter() {
        let substance = object.material.pick_substance();

        for _ in 0..time_data.multi_step {
            match &substance {
                Substance::Element(element, n) => {
                    if element.isotopes[n].is_usable {
                        let volume = transform.scale.x * transform.scale.y * transform.scale.z;
                        let weight = volume * element.density;
                        let estimated_decays = element.isotopes[n].activity.unwrap()
                            * weight
                            * time_data.time_step_calc;

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
                            commands.spawn((
                                TransformBundle::from_transform(Transform::from_translation(
                                    global_transform.translation() + pos_offset,
                                )),
                                Particle {
                                    // these have energy as velocity
                                    energy: 0.0,
                                    particle_type: particle_type.clone(),
                                },
                                Velocity(
                                    velocity_direction
                                        * energy_to_velocity(decay.decay_energy, particle_type),
                                ),
                                VisibilityBundle::default(),
                            ));

                            // spawn gamma ray
                            if let Some(gamma_energy) = decay.gamma_energy {
                                commands.spawn((
                                    TransformBundle::from_transform(Transform::from_translation(
                                        transform.clone().translation + pos_offset,
                                    )),
                                    Particle {
                                        energy: gamma_energy,
                                        particle_type: ParticleType::Gamma,
                                    },
                                    Velocity(velocity_direction * LIGHT_SPEED),
                                    VisibilityBundle::default(),
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn process_particles(
    time_data: ResMut<TimeData>,

    ambient_query: Query<&AmbientMaterial>,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &mut Particle), Without<Object>>,
    mut object_query: Query<(&mut Object, &Transform, &GlobalTransform), Without<Particle>>,

    par_commands: ParallelCommands,
) {
    if time_data.halted {
        return;
    }

    let ambient_material = ambient_query.iter().next().unwrap();

    let objects = object_query
        .iter_mut()
        .map(|q| (q, AtomicF32::new(0.0)))
        .collect::<Vec<_>>();

    query
        .par_iter_mut()
        .for_each_mut(|(entity, mut transform, mut velocity, mut particle)| {
            for _ in 0..time_data.multi_step {
                // move particle
                let move_step = velocity.0 * time_data.time_step_move as f32;
                transform.translation += move_step;

                // collide particle

                let mut hit_substance = None;
                let mut hit_obstacle = None;

                for ((object, object_transform, object_global_transform), absorbed_energy) in
                    &objects
                {
                    let par_pos = transform.translation;
                    let obj_pos = object_global_transform.translation();
                    let obj_scale = object_transform.scale;

                    // check for hit
                    if par_pos.x > obj_pos.x - obj_scale.x / 2.0
                        && par_pos.x < obj_pos.x + obj_scale.x / 2.0
                        && par_pos.y > obj_pos.y - obj_scale.y / 2.0
                        && par_pos.y < obj_pos.y + obj_scale.y / 2.0
                        && par_pos.z > obj_pos.z - obj_scale.z / 2.0
                        && par_pos.z < obj_pos.z + obj_scale.z / 2.0
                    {
                        let substance = object.material.pick_substance();

                        hit_substance = Some(substance);
                        hit_obstacle = Some(absorbed_energy);
                    }
                }

                if hit_substance.is_none() {
                    hit_substance = Some(ambient_material.material.pick_substance());
                }

                // apply material
                if let Some(substance) = hit_substance {
                    if let Some(stopping_powers) = substance.stopping_powers(particle.particle_type)
                    {
                        let energy = match particle.particle_type {
                            ParticleType::Gamma => particle.energy,
                            _ => velocity_to_energy(
                                velocity.0.clone().length(),
                                particle.particle_type,
                            ),
                        };

                        // MeV/m or 1/m
                        let stopping_power = pick_stopping_power(stopping_powers, energy);

                        let energy_transfer = calculate_energy_transfer(
                            stopping_power,
                            particle.particle_type,
                            energy,
                            move_step.length(),
                        );

                        // add to obstacle
                        if let Some(absorbed_energy) = hit_obstacle {
                            absorbed_energy.fetch_add(
                                // account for equivalent dose
                                match particle.particle_type {
                                    ParticleType::Alpha => energy_transfer * 20.0,
                                    _ => energy_transfer,
                                },
                                Ordering::Relaxed,
                            );
                        }

                        let new_energy = (energy - energy_transfer).max(0.0);

                        match particle.particle_type {
                            ParticleType::Gamma => {
                                particle.energy = new_energy;
                            }
                            _ => {
                                velocity.0 = velocity.0.normalize()
                                    * energy_to_velocity(new_energy, particle.particle_type)
                            }
                        }
                    }
                }

                if particle.energy < 0.1 || velocity.0.length() < 0.1 {
                    par_commands.command_scope(|mut commands| {
                        commands.entity(entity).despawn();
                    });
                    break;
                }
            }
        });

    for ((mut obstacle, _, _), absorbed_energy) in objects {
        obstacle.absorbed_energy += absorbed_energy.load(Ordering::Relaxed);
    }
}

fn pick_stopping_power(stopping_powers: &StoppingPower, energy: f32) -> f32 {
    for (stop_energy, stopping_power) in stopping_powers {
        if *stop_energy > energy {
            return *stopping_power;
        }
    }
    return stopping_powers.last().unwrap().1;
}

fn energy_to_velocity(energy: f32, particle_type: ParticleType) -> f32 {
    // TODO: account for relavistiuc movement (thanks Einstein...)
    let mass = match particle_type {
        ParticleType::Alpha => *ALPHA_MASS,
        _ => *ELECTRON_MASS,
    };
    (2.0 * energy * *EV_CONVERSION / mass).sqrt()
}

fn velocity_to_energy(velocity: f32, particle_type: ParticleType) -> f32 {
    // TODO: account for relavistiuc movement (thanks Einstein...)
    let mass = match particle_type {
        ParticleType::Alpha => *ALPHA_MASS,
        _ => *ELECTRON_MASS,
    };
    ((mass * velocity.powi(2)) / 2.0) / *EV_CONVERSION
}

// fn test() {
//     let energy = 1_000_000.0;

//     let velocity = energy_to_velocity(energy, ParticleType::Alpha);
//     dbg!(velocity);
//     let new_energy = velocity_to_energy(velocity, ParticleType::Alpha);
//     dbg!(new_energy);

//     dbg!(velocity_to_energy(0.0, ParticleType::Alpha));
//     dbg!(energy_to_velocity(0.0, ParticleType::Alpha));
// }

fn calculate_energy_transfer(
    stopping_power: f32,
    particle_type: ParticleType,
    energy: f32,
    distance: f32,
) -> f32 {
    match particle_type {
        // gammas either are unaffected or completely gone
        ParticleType::Gamma => {
            if std::f32::consts::E.powf(-1.0 * stopping_power * distance) < fastrand::f32() {
                // transfer all energy if "hit"
                energy
            } else {
                // none if no "hit"
                0.0
            }
        }
        _ => stopping_power * distance,
    }
}

pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(
            DefaultPlugins
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "spawn=trace,wgpu_core=warn,wgpu_hal=warn".to_string(),
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Radiation Simulation".into(),
                        fit_canvas_to_parent: true,
                        #[cfg(not(debug_assertions))]
                        canvas: Some("#maincanvas".to_owned()),
                        resizable: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_system(bevy::window::close_on_esc)
        .add_plugin(RadiationSim)
        .run();
}
