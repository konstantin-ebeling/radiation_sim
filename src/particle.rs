use std::sync::atomic::Ordering;

use atomic_float::AtomicF32;
use bevy::{prelude::*, render::view::NoFrustumCulling};

use crate::{
    element, render, CurrentEnv, MaterialData, RadiationSimData, StoppingPower, Substance,
    ALPHA_MASS, ELECTRON_MASS, EV_CONVERSION, LIGHT_SPEED, LIGHT_SPEED_SQ,
};

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

#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Object {
    pub material: MaterialData,
    pub absorbed_energy: f32,
}

#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect(Component)]
pub struct LinearSpawner {
    pub alpha_rate: f32,
    pub beta_rate: f32,
    pub gamma_rate: f32,
    pub particle_energy: f32,
}

#[derive(Debug, Clone, PartialEq, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Velocity(Vec3);

#[derive(Debug, Resource)]
pub struct TimeData {
    pub time_step_calc: f32,
    pub time_step_move: f32,
    pub multi_step: usize,
    pub halted: bool,
    pub time_passed: f32,
}

pub struct RadiationSimParticle;

impl Plugin for RadiationSimParticle {
    fn build(&self, app: &mut App) {
        app.add_plugin(RadiationSimData)
            .add_plugin(render::ParticleRenderPlugin)
            .insert_resource(TimeData {
                time_step_move: (10f32).powi(-12),
                time_step_calc: (10f32).powi(-11),
                multi_step: 16,
                halted: false,
                time_passed: 0.0,
            })
            .add_event::<ResetParticles>()
            .add_startup_system(setup)
            .add_system(tick_time)
            .add_system(spawn_object_particles.in_set(OnUpdate(CurrentEnv::Sandbox)))
            .add_system(spawn_linear_particles.in_set(OnUpdate(CurrentEnv::Experiment)))
            .add_system(reset_particles)
            .add_system(process_particles);
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        meshes.add(Mesh::from(shape::Cube { size: 0.003 })),
        Transform::default(),
        GlobalTransform::default(),
        render::InstanceMaterialData(vec![]),
        Visibility::default(),
        ComputedVisibility::default(),
        NoFrustumCulling,
    ));
}

fn tick_time(mut time_data: ResMut<TimeData>) {
    if time_data.halted {
        return;
    }

    time_data.time_passed += time_data.time_step_calc * time_data.multi_step as f32;
}

fn spawn_object_particles(
    time_data: ResMut<TimeData>,
    query: Query<(&Transform, &GlobalTransform, &Object)>,
    mut commands: Commands,
) {
    if time_data.halted {
        return;
    }

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
                                    energy: 1.0,
                                    particle_type,
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
                                        transform.translation + pos_offset,
                                    )),
                                    Particle {
                                        energy: gamma_energy,
                                        particle_type: ParticleType::Gamma,
                                    },
                                    Velocity(velocity_direction * LIGHT_SPEED as f32),
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

fn spawn_linear_particles(
    time_data: ResMut<TimeData>,
    query: Query<(&Transform, &GlobalTransform, &LinearSpawner)>,
    mut commands: Commands,
) {
    if time_data.halted {
        return;
    }

    for (transform, global_transform, spawner) in query.iter() {
        for _ in 0..time_data.multi_step {
            let particle_types = [
                (ParticleType::Alpha, spawner.alpha_rate),
                (ParticleType::Electron, spawner.beta_rate),
                (ParticleType::Gamma, spawner.gamma_rate),
            ];
            for (particle_type, rate) in particle_types {
                let estimated_decays = rate * time_data.time_step_calc;

                let decays = estimated_decays.floor() as usize
                    + if (estimated_decays - estimated_decays.floor()) > fastrand::f32() {
                        1
                    } else {
                        0
                    };

                for _ in 0..decays {
                    let velocity_direction = Vec3::new(1.0, 0.0, 0.0);

                    let pos_offset = Vec3::new(
                        transform.scale.x * 0.5,
                        transform.scale.y * (fastrand::f32() - 0.5),
                        transform.scale.z * (fastrand::f32() - 0.5),
                    );

                    if !matches!(particle_type, ParticleType::Gamma) {
                        commands.spawn((
                            TransformBundle::from_transform(Transform::from_translation(
                                global_transform.translation() + pos_offset,
                            )),
                            Particle {
                                // these have energy as velocity
                                energy: 1.0,
                                particle_type,
                            },
                            Velocity(
                                velocity_direction
                                    * energy_to_velocity(spawner.particle_energy, particle_type),
                            ),
                            VisibilityBundle::default(),
                        ));
                    } else {
                        commands.spawn((
                            TransformBundle::from_transform(Transform::from_translation(
                                transform.translation + pos_offset,
                            )),
                            Particle {
                                energy: spawner.particle_energy,
                                particle_type: ParticleType::Gamma,
                            },
                            Velocity(velocity_direction * LIGHT_SPEED as f32),
                            VisibilityBundle::default(),
                        ));
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct ResetParticles;

fn reset_particles(
    particle_query: Query<Entity, With<Particle>>,
    mut object_query: Query<&mut Object>,
    mut commands: Commands,
    mut events: EventReader<ResetParticles>,
) {
    if !events.is_empty() {
        events.clear();
        particle_query.iter().for_each(|e| {
            commands.entity(e).despawn();
        });

        object_query.iter_mut().for_each(|mut object| {
            object.absorbed_energy = 0.0;
        });
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
                let move_step = velocity.0 * time_data.time_step_move;
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
                            _ => velocity_to_energy(velocity.0.length(), particle.particle_type),
                        };

                        // eV/m or 1/m
                        let stopping_power = pick_stopping_power(stopping_powers, energy);

                        let energy_transfer = match particle.particle_type {
                            // gammas either are unaffected or completely gone
                            ParticleType::Gamma => {
                                if std::f32::consts::E
                                    .powf(-1.0 * stopping_power * move_step.length())
                                    < fastrand::f32()
                                {
                                    // transfer all energy if "hit"
                                    energy
                                } else {
                                    // none if no "hit"
                                    0.0
                                }
                            }
                            _ => stopping_power * move_step.length(),
                        };

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

                if particle.energy < 0.1 || velocity.0.length() < 10.0 {
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
    let mass = match particle_type {
        ParticleType::Electron => *ELECTRON_MASS,
        _ => *ALPHA_MASS,
    };

    let k = ((energy as f64 * *EV_CONVERSION) / (mass * LIGHT_SPEED_SQ)) + 1.0;
    let k_sq = k.powi(2);

    ((LIGHT_SPEED * (k_sq - 1.0).sqrt()) / k) as f32
}

fn velocity_to_energy(velocity: f32, particle_type: ParticleType) -> f32 {
    let mass = match particle_type {
        ParticleType::Electron => *ELECTRON_MASS,
        _ => *ALPHA_MASS,
    };

    let k = 1.0 / (1.0 - (velocity as f64 / LIGHT_SPEED).powi(2)).sqrt();

    ((k - 1.0) * mass * LIGHT_SPEED_SQ / *EV_CONVERSION) as f32
}
