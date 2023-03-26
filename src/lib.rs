use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

pub mod constants;
pub use constants::*;
mod data_reading;
use data_reading::*;
mod env;
use env::*;
pub mod material;
use material::*;
mod particle;
use particle::*;
mod render;
mod ui;
use ui::*;

#[derive(Debug, Resource)]
pub struct InterfaceState {
    advanced: bool,
    edit_objects: bool,
}

#[derive(Debug, Resource, Default)]
pub struct AssetHandles {
    cube_mesh: Option<Handle<Mesh>>,
    grey_material: Option<Handle<StandardMaterial>>,
    light_grey_material: Option<Handle<StandardMaterial>>,
    checkerboard_material: Option<Handle<StandardMaterial>>,
}
pub struct RadiationSim;

impl Plugin for RadiationSim {
    fn build(&self, app: &mut App) {
        app.add_plugin(RadiationSimUI)
            .add_plugin(RadiationSimEnv)
            .add_plugin(RadiationSimParticle)
            .insert_resource(InterfaceState {
                // in debug builds show advanced default
                advanced: cfg!(debug_assertions),
                edit_objects: cfg!(debug_assertions),
            })
            .init_resource::<AssetHandles>()
            .insert_resource(AmbientLight {
                brightness: 0.1,
                color: Color::rgb(1.0, 1.0, 1.0),
            })
            .add_startup_system(setup)
            .add_system(move_camera);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut asset_handles: ResMut<AssetHandles>,
    asset_server: Res<AssetServer>,
) {
    // ------ World ------

    let mut light_transform = Transform::default();
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
        transform: Transform::from_xyz(-0.2, 0.5, 0.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // assets

    let cube_mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    asset_handles.cube_mesh = Some(cube_mesh);
    let grey_material = materials.add(Color::rgb(0.6, 0.6, 0.6).into());
    asset_handles.grey_material = Some(grey_material);
    let light_grey_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    asset_handles.light_grey_material = Some(light_grey_material);
    let checker_board_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(asset_server.load("checkerboard.png")),

        ..Default::default()
    });
    asset_handles.checkerboard_material = Some(checker_board_material);
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
                    0.2
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

pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(
            DefaultPlugins
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::INFO,
                    filter: "spawn=trace,wgpu_core=warn,wgpu_hal=error,bevy_ecs=error".to_string(),
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
