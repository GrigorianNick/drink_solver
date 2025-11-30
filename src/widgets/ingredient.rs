use std::{cell::RefCell, rc::Rc};

use egui::Widget;

use crate::ingredients::IngredientStore;

pub struct IngredientWidget {
    ingredient_store: Rc<RefCell<IngredientStore>>,
    selected_name: String
}

impl IngredientWidget {
    pub fn new(store: Rc<RefCell<IngredientStore>>) -> IngredientWidget {
        IngredientWidget { ingredient_store: store, selected_name: "".into() }
    }
}

impl Widget for &mut IngredientWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Max).with_cross_justify(true), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    for name in self.ingredient_store.borrow().get_ingredient_names() {
                        ui.selectable_value(&mut self.selected_name, name.clone(), name.clone());
                    }
                })
            });
            ui.separator();
            if let Some(ingredient) = self.ingredient_store.borrow().get_ingredient(&self.selected_name) {
                ui.vertical(|ui| {
                    ui.heading(ingredient.name);
                    ui.separator();
                    ui.label(format!("Quality: {}", ingredient.quality.to_string()));
                    ui.separator();
                    for tag in ingredient.tags {
                        ui.label(tag.value);
                    }
                });
            }
        }).response
    }
}