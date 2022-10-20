use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::{Constants, InterfaceState};

pub struct RadiationSimUI;

impl Plugin for RadiationSimUI {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(render_ui)
            .add_system(render_legend);
    }
}

fn render_ui(
    mut egui_context: ResMut<EguiContext>,
    constants: Res<Constants>,
    mut interface_state: ResMut<InterfaceState>,
) {
    egui::Window::new("Simulation von Radioaktivität").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Virtuelle Umgebung");
        ui.label("Anzahl simulierte Teilchen: 587");
        ui.label("Anzahl Hindernisse: 1");
        ui.button("Bearbeiten");
        ui.button("Simulation pausieren");
        ui.separator();

        ui.heading("Messwerte");
        ui.label("Energiedosis: 0,161 mGy");
        ui.label("Äquivalenzdosis: 0,165 mSv");
        ui.button("Zurücksetzen");
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

    egui::Window::new("Elemente").show(egui_context.ctx_mut(), |ui| {
        for (z, element) in &constants.elements {
            ui.collapsing(&element.name, |ui| {
                ui.label(z.to_string());
                ui.label(&element.symbol);

                for isotope in &element.isotopes {
                    ui.label(format!(
                        "{}{} {}",
                        element.symbol,
                        isotope.1.z + isotope.1.n,
                        match isotope.1.half_life {
                            Some(half_life) => format!("{}s", half_life),
                            None => "Stable".to_owned(),
                        }
                    ));
                }
            });
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
