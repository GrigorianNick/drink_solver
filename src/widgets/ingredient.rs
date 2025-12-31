use std::{cell::RefCell, rc::Rc};

use egui::{
    Button, CentralPanel, DragValue, Grid, SidePanel, TopBottomPanel, Widget, special_emojis,
};

use crate::{ingredient_store::IngredientStore, store::Store};

pub struct IngredientWidget {
    ingredient_store: Rc<RefCell<IngredientStore>>,
    selected_ingredient: uuid::Uuid,
}

impl IngredientWidget {
    pub fn new(store: Rc<RefCell<IngredientStore>>) -> IngredientWidget {
        IngredientWidget {
            ingredient_store: store,
            selected_ingredient: uuid::Uuid::nil(),
        }
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
                        let mut entries = binding.get_ingredient_entries();
                        entries.sort_by_key(|e| e.1.name.to_lowercase());
                        for (id, entry) in entries {
                            ui.selectable_value(
                                &mut self.selected_ingredient,
                                id,
                                entry.name.clone(),
                            );
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
                    })
                    .response
            })
        });
        if self.selected_ingredient != uuid::Uuid::nil() {
            TopBottomPanel::bottom("ingredient_footer").show_inside(ui, |ui| {
                if ui.button("Delete entry").clicked() {
                    self.ingredient_store
                        .borrow_mut()
                        .deregister(self.selected_ingredient);
                    self.selected_ingredient = uuid::Uuid::nil();
                }
            });
        }
        CentralPanel::default()
            .show_inside(ui, |ui| {
                if let Some(ingredient) = self
                    .ingredient_store
                    .borrow()
                    .get_entry(self.selected_ingredient)
                {
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
            })
            .response
    }
}
