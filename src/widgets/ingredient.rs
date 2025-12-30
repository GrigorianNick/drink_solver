use std::{cell::RefCell, rc::Rc};

use egui::{Button, CentralPanel, DragValue, Grid, SidePanel, Widget, special_emojis};

use crate::{ingredient_store::IngredientStore, store::Store};

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
        SidePanel::left("ingredient_side_panel_list").show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                Grid::new("ingredient_widget_sidepane_list")
                    .striped(true)
                    .show(ui, |ui| {
                        let mut binding = self.ingredient_store.borrow_mut();
                        let mut entries = binding.get_entries_mut();
                        entries.sort_by_key(|e| e.name.to_lowercase());
                        for entry in entries {
                            ui.selectable_value(&mut self.selected_name, entry.name.clone(), entry.name.clone());
                            ui.horizontal(|ui| {
                                if ui.add_enabled(entry.stock > 0, Button::new("-")).clicked() {
                                    entry.stock -= 1;
                                }
                                ui.add(DragValue::new(&mut entry.stock));
                                if ui.button("+").clicked() {
                                    entry.stock += 1;
                                }
                            });
                            ui.end_row();
                        }
                    }).response
            })
        });
        CentralPanel::default().show_inside(ui, |ui| {
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