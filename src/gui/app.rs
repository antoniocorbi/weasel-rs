// Copyright (C) 2025  Antonio-Miguel Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use eframe::App;
// -- Uses: ---------------------------------------------------------------
use egui::{Align, Color32, TextEdit};
use signals2::*;
use weasel_rs::libweasel::{
    arguments, charset,
    chromosome::{Chromosome, EvolvingChromosome, StandardChromosome},
    gene::{Gene, GeneCreationExt, GeneExt, MutableGene},
};

// -- Structs: ------------------------------------------------------------
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WeaselApp {
    // Example stuff:
    // #[serde(skip)] // This how you opt-out of serialization of a field
    sentence: String,

    // #[serde(skip)] // This how you opt-out of serialization of a field
    zoom: f32,
    // #[serde(skip)] // This how you opt-out of serialization of a field
    mrate: f64,
    // #[serde(skip)] // This how you opt-out of serialization of a field
    ncopies: u32,

    // The chromosome to play with
    ec: Chromosome<MutableGene>,

    // Enable stop button
    enable_stop: bool,

    // Enable start button
    enable_start: bool,
    // Enable text edition
    enable_edition: bool,
}

enum Evolving {
    YES,
    NO,
}

// -- Traits: -------------------------------------------------------------

trait Ui {
    fn draw_top_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
    fn draw_central_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
    fn draw_bottom_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
    fn update_enable_ui_status(&mut self, evolving: Evolving);
}

// -- Impl blocks: --------------------------------------------------------
impl Default for WeaselApp {
    fn default() -> Self {
        let sentence = "Hello World!".to_owned();
        let mrate = 0.07;
        let ncopies = 600;
        let mut ec = EvolvingChromosome::new(sentence.clone(), ncopies).with_mr(mrate);

        // ec.create_random_genes();

        Self {
            // Example stuff:
            sentence,
            zoom: 2.0,
            mrate,
            ncopies,
            ec,
            enable_start: true,
            enable_stop: false,
            enable_edition: true,
        }
    }
}

impl WeaselApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        return Default::default();

        // if let Some(storage) = cc.storage {
        //     eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        // } else {
        //     Default::default()
        // }
    }
}

impl Ui for WeaselApp {
    fn draw_top_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
    }

    fn update_enable_ui_status(&mut self, evolving: Evolving) {
        match evolving {
            Evolving::YES => {
                self.enable_stop = true;
                self.enable_start = false;
                self.enable_edition = false;
            }
            Evolving::NO => {
                self.enable_stop = false;
                self.enable_start = true;
                self.enable_edition = true;
            }
        }
    }

    fn draw_central_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Weasel GUI");
            ui.separator();

            // Sentence
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Sentence: ")
                        .color(Color32::LIGHT_GREEN)
                        .underline(),
                );
                //ui.text_edit_singleline(&mut self.sentence);
                ui.add_enabled(
                    self.enable_edition,
                    egui::TextEdit::singleline(&mut self.sentence),
                );

                // If the user is changing the input sentence then
                // update it into the evolving chromosome 'ec'.
                if self.sentence != self.ec.target() {
                    let ts = self.sentence.clone();
                    self.ec.set_target(ts);
                }
            });

            // Mutation rate
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Mutation rate: ")
                        .color(Color32::LIGHT_GREEN)
                        .underline(),
                );
                ui.add(
                    egui::DragValue::new(&mut self.mrate)
                        .range(0.0..=1.0)
                        .speed(0.025)
                        .max_decimals(2),
                );
                self.ec.set_mr(self.mrate);
            });

            // Number of copies in each generation
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Number of copies: ")
                        .color(Color32::LIGHT_GREEN)
                        .underline(),
                );
                ui.add(
                    egui::DragValue::new(&mut self.ncopies)
                        .range(0..=5000)
                        .speed(10),
                );
                self.ec.set_ncopies(self.ncopies);
            });

            // Action buttons
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                //ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        self.enable_start,
                        egui::Button::new(egui::RichText::new("Start").color(Color32::YELLOW)),
                    )
                    .clicked()
                {
                    // self.enable_stop = true;
                    // self.enable_start = false;
                    // self.enable_edition = false;
                    self.update_enable_ui_status(Evolving::YES);

                    self.ec.init_evolution_data();
                }
                if ui
                    .add_enabled(
                        self.enable_stop,
                        egui::Button::new(egui::RichText::new("Stop").color(Color32::RED)),
                    )
                    .clicked()
                {
                    // self.enable_stop = false;
                    // self.enable_start = true;
                    // self.enable_edition = true;
                    self.update_enable_ui_status(Evolving::NO);
                }

                let spacing = ui.available_width() - 300.0; // 60.0 es un estimado del ancho del segundo botón
                ui.add_space(spacing);

                ui.label(
                    egui::RichText::new(format!("BF: {}", self.ec.bestfit()))
                        .strong()
                        .color(Color32::LIGHT_GREEN)
                        .underline(),
                );

                let spacing = ui.available_width() - 120.0; // 60.0 es un estimado del ancho del segundo botón
                ui.add_space(spacing);

                ui.label(
                    egui::RichText::new(format!("Generation: {}", self.ec.get_current_gen()))
                        .strong()
                        .color(Color32::LIGHT_YELLOW)
                        .underline(),
                );
            });

            ui.separator();

            // Loop the chromosome evolution
            if self.enable_stop && self.ec.bestfit() > 0 {
                self.ec.next_evolution_generation();
                // println!(
                //     "gen: {}, bf: {}, ncopies: {}, mr: {}",
                //     self.ec.get_current_gen(),
                //     self.ec.bestfit(),
                //     self.ec.ncopies(),
                //     self.ec.mr()
                // );
            } else if self.ec.bestfit() == 0 {
                // self.enable_stop = false;
                // self.enable_start = true;
                // self.enable_edition = true;
                self.update_enable_ui_status(Evolving::NO);
            }

            // As seen on: https://docs.rs/egui/latest/egui/widgets/text_edit/struct.TextEdit.html
            // let mut s: &str = self.sentence.as_str();
            let mut s = self.ec.get_genes();
            ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut s));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
                ui.separator();
            });
        });
    }

    fn draw_bottom_ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("bottom_panel")
            .show_separator_line(true)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    powered_by_egui_and_eframe(ui);
                    egui::warn_if_debug_build(ui);
                    egui::github_link_file!(
                        "https://github.com/emilk/eframe_template/blob/main/",
                        "Source code."
                    );
                });
            });
    }
}

impl eframe::App for WeaselApp {
    /// Called by the framework to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // set zoom-factor.
        ctx.set_zoom_factor(self.zoom);

        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        self.draw_top_ui(ctx, frame);

        // Central panel section
        self.draw_central_ui(ctx, frame);

        // Bottom panel section
        //self.draw_bottom_ui(ctx, frame);

        // Continuous update
        ctx.request_repaint();
    }
}

// -- Free functions: -----------------------------------------------------
fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
