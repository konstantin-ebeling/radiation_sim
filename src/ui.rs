use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::Constants;

pub struct RadiationSimUI;

impl Plugin for RadiationSimUI {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(render_ui);
    }
}

fn render_ui(mut egui_context: ResMut<EguiContext>, constants: Res<Constants>) {
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
        ui.label("Benutzen Sie die rechte Maustaste um sich umzuschauen und das Scroll-Rad um sich vor und zurück zu bewegen.")
    });

    egui::Window::new("Legende")
        .id(egui::Id::new("legend"))
        .show(egui_context.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.color_edit_button_rgb(&mut [1.0, 0.0, 0.0]);
                ui.label("α-Teilchen");
            });
            ui.horizontal(|ui| {
                ui.color_edit_button_rgb(&mut [0.1, 0.9, 0.1]);
                ui.label("Elektron (β-Strahlung)");
            });
            ui.horizontal(|ui| {
                ui.color_edit_button_rgb(&mut [0.9, 0.9, 0.0]);
                ui.label("Photon (γ-Strahlung)");
            });
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
