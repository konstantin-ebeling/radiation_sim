use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::{
    presets, AssetHandles, Human, HumanRoot, InterfaceState, Object, Particle, SubstanceData,
    TimeData,
};

pub struct RadiationSimUI;

impl Plugin for RadiationSimUI {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(render_main_ui)
            .add_system(render_legend)
            .add_system(render_object_editor);
    }
}

fn render_main_ui(
    mut egui_context: ResMut<EguiContext>,
    mut time_data: ResMut<TimeData>,
    mut interface_state: ResMut<InterfaceState>,

    particle_query: Query<(Entity, &Particle)>,
    mut object_query: Query<&mut Object>,

    mut commands: Commands,
) {
    egui::Window::new("Simulation von Radioaktivität").anchor(egui::Align2::LEFT_TOP, [10.0, 10.0]).show(egui_context.ctx_mut(), |ui| {
        ui.heading("Virtuelle Umgebung");
        ui.label(format!("Anzahl simulierte Teilchen: {}", particle_query.iter().len()));
        if !interface_state.edit_objects {
            if ui.button("Bearbeiten").clicked() {
                interface_state.edit_objects = true;
            }
        } else {
            if ui.button("Bearbeitung stoppen").clicked() {
                interface_state.edit_objects = false;
            }
        }
        ui.separator();

        ui.heading("Zeit");
        ui.label("Zeit Faktor: 10^-12");

        if !time_data.halted {
            if ui.button("Simulation pausieren").clicked() {
                time_data.halted = true;
            }
        } else {
            if ui.button("Simulation fortsetzen").clicked() {
                time_data.halted = false;
            }
        }

        ui.separator();

        ui.heading("Messwerte");
        ui.label("Energiedosis: 0 mGy");
        ui.label("Äquivalenzdosis: 0 mSv");
        if ui.button("Zurücksetzen").clicked() {
            particle_query.iter().for_each(|(e, _)| {
                commands.entity(e).despawn();
            });

            object_query.iter_mut().for_each(|mut object| {
                object.absorbed_energy = 0.0;
            });
        }
        ui.separator();

        ui.heading("Steuerung");
        if interface_state.advanced {
            ui.label("Benutzen Sie die rechte Maustaste um sich umzuschauen und das Scroll-Rad um sich vor und zurück zu bewegen.");
            if ui.button("Zur vereinfachten Steuerung wechseln").clicked() {
                interface_state.advanced = false;
            }
        } else {
            ui.label("Benutzen Sie die linke Maustaste oder tippen um sich umzuschauen");
            if ui.button("Zur erweiterten Steuerung wechseln").clicked() {
                interface_state.advanced = true;
            }
        }
    });
}

fn render_legend(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Legende")
        .anchor(egui::Align2::LEFT_BOTTOM, [10.0, -10.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|mut ui| {
                egui::color_picker::show_color(
                    &mut ui,
                    egui::Color32::from_rgb(255, 0, 0),
                    [13.0, 13.0].into(),
                );
                ui.label("α-Teilchen");
            });
            ui.horizontal(|mut ui| {
                egui::color_picker::show_color(
                    &mut ui,
                    egui::Color32::from_rgb(25, 230, 25),
                    [13.0, 13.0].into(),
                );
                ui.label("Elektron (β-Strahlung)");
            });
            ui.horizontal(|mut ui| {
                egui::color_picker::show_color(
                    &mut ui,
                    egui::Color32::from_rgb(230, 230, 0),
                    [13.0, 13.0].into(),
                );
                ui.label("Photon (γ-Strahlung)");
            });
        });
}

fn render_object_editor(
    mut egui_context: ResMut<EguiContext>,
    mut interface_state: ResMut<InterfaceState>,
    mut set: ParamSet<(
        Query<(Entity, &mut Object, &mut Name, &mut Transform), Without<Human>>,
        Query<&mut Transform, With<HumanRoot>>,
    )>,
    asset_handles: Res<AssetHandles>,
    substance_data: Res<SubstanceData>,

    mut commands: Commands,
) {
    egui::Window::new("Objekt Bearbeitung")
        .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
        .open(&mut interface_state.edit_objects)
        .show(egui_context.ctx_mut(), |ui| {
            let mut i = 1;
            for (entity, mut object, mut name, mut transform) in set.p0().iter_mut() {
                ui.collapsing(name.clone().as_str(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        name.mutate(|n| {
                            ui.text_edit_singleline(n);
                        })
                    });

                    position_editor(ui, &mut transform);

                    ui.label("Größe (x, y, z)");
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut transform.scale.x)
                                .clamp_range(0..=100)
                                .speed(0.05),
                        );
                        // y/z flipped to match with mathematicl norm
                        ui.add(
                            egui::DragValue::new(&mut transform.scale.z)
                                .clamp_range(0..=100)
                                .speed(0.05),
                        );
                        ui.add(
                            egui::DragValue::new(&mut transform.scale.y)
                                .clamp_range(0..=100)
                                .speed(0.05),
                        );
                    });

                    ui.label("Material");
                    ui.label(format!("{:?}", object.material.parts[0].1.name()));

                    ui.label(format!("Absorbierte Energie: {}eV", object.absorbed_energy));

                    if ui.button("Entfernen").clicked() {
                        commands.entity(entity).despawn();
                    }
                });

                i += 1;
            }

            if ui.button("Neues Objekt").clicked() {
                commands
                    .spawn()
                    .insert(Name::new(format!("Objekt {}", i)))
                    .insert_bundle(PbrBundle {
                        material: asset_handles.grey_material.as_ref().unwrap().clone(),
                        mesh: asset_handles.cube_mesh.as_ref().unwrap().clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0)
                            .with_scale(Vec3::new(1.0, 1.0, 1.0)),
                        ..Default::default()
                    })
                    .insert(Object {
                        material: presets::pb208(&substance_data),
                        ..Default::default()
                    });
            }

            ui.collapsing("Mensch", |ui| {
                let mut human_query = set.p1();
                let mut transform = human_query.iter_mut().next().unwrap();

                position_editor(ui, &mut transform);
            });
        });
}

fn position_editor(ui: &mut egui::Ui, transform: &mut Transform) {
    ui.label("Position (x, y, z)");
    ui.horizontal(|ui| {
        ui.add(
            egui::DragValue::new(&mut transform.translation.x)
                .clamp_range(-50..=50)
                .speed(0.1),
        );
        // y/z flipped to match with mathematicl norm
        ui.add(
            egui::DragValue::new(&mut transform.translation.z)
                .clamp_range(-50..=50)
                .speed(0.1),
        );
        ui.add(
            egui::DragValue::new(&mut transform.translation.y)
                .clamp_range(-50..=50)
                .speed(0.1),
        );
    });
}

// egui::Window::new("Elemente").show(egui_context.ctx_mut(), |ui| {
//     for (z, element) in &constants.elements {
//         ui.collapsing(&element.name, |ui| {
//             ui.label(z.to_string());
//             ui.label(&element.symbol);

//             for isotope in &element.isotopes {
//                 ui.label(format!(
//                     "{}{} {}",
//                     element.symbol,
//                     isotope.1.z + isotope.1.n,
//                     match isotope.1.half_life {
//                         Some(half_life) => format!("{}s", half_life),
//                         None => "Stable".to_owned(),
//                     }
//                 ));
//             }
//         });
//     }
// });
