#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod ingredient;
mod recipie;
mod recipie_builder;
mod recipie_store;
mod widgets;
mod ingredient_builder;
mod builder;
mod store;
mod ingredient_store;
mod ingredient_selector_builder;
mod component_builder;

use std::io::Read;

use eframe::egui;
use egui::{IconData, Image, ImageSource, Style, Visuals, emath::interpolation_factor, epaint::image, include_image};
use ::image::load_from_memory;

fn main() -> eframe::Result {
    let icon_bytes = include_bytes!("../icon.png");
    let (rgba, width, height) = {
        let img = load_from_memory(icon_bytes).expect("Missing icon.png!");
        (img.as_rgba8().expect("Cannot convert icon to rgba8!").to_vec(), img.width(), img.height())
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(std::sync::Arc::new(
                IconData {
                    rgba,
                    width,
                    height
                }
            )),
        ..Default::default()
    };
    eframe::run_native(
        "DrinkSolver",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::<crate::widgets::main_widget::MyApp>::default())
        }),
    )
}