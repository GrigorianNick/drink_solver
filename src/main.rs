#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod builder;
mod component_builder;
mod ingredient;
mod ingredient_builder;
mod ingredient_selector_builder;
mod ingredient_store;
mod measure;
mod recipie;
mod recipie_builder;
mod recipie_store;
mod store;
mod widgets;

use ::image::load_from_memory;
use eframe::egui;
use egui::IconData;

fn main() -> eframe::Result {
    let icon_bytes = include_bytes!("../icon.png");
    let (rgba, width, height) = {
        let img = load_from_memory(icon_bytes).expect("Missing icon.png!");
        (
            img.as_rgba8()
                .expect("Cannot convert icon to rgba8!")
                .to_vec(),
            img.width(),
            img.height(),
        )
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_icon(std::sync::Arc::new(IconData {
            rgba,
            width,
            height,
        })),
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
