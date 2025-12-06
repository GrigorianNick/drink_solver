#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod ingredients;
mod recipie;
mod recipie_builder;
mod recipie_store;
mod widgets;
mod builder;
mod ingredient_builder;

use eframe::egui;

use crate::ingredients::IngredientStore;
use crate::recipie_store::RecipieStore;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
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