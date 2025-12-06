use std::{cell::RefCell, rc::Rc};

use egui::{ComboBox, Widget, emath::easing::quadratic_in};
use strum::IntoEnumIterator;

use crate::{builder::Builder, ingredient_builder::IngredientBuilder, ingredients::{Ingredient, IngredientStore, Quality}};

#[derive(Default, Clone)]
pub struct  CreateIngredientWidget {
    builder: IngredientBuilder,
    store: Rc<RefCell<IngredientStore>>
}

impl CreateIngredientWidget {
    pub fn new(store: Rc<RefCell<IngredientStore>>) -> Self {
        CreateIngredientWidget { builder: IngredientBuilder::default(), store: store }
    }
}

impl Widget for &mut CreateIngredientWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.text_edit_singleline(&mut self.builder.name);
        ui.separator();
        ui.label("Quality");
        ComboBox::from_id_salt("CreateIngredientWidgetQuaulity")
            .selected_text(self.builder.quality.to_string())
            .show_ui(ui, |ui| {
            for quality in Quality::iter() {
                ui.selectable_value(&mut self.builder.quality,  quality, quality.to_string());
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                self.store.borrow_mut().register_ingredient(self.builder.build());
                self.builder.clear();
            }
            if ui.button("Reset").clicked() {
                self.builder.clear();
            }
        }).response
    }
}