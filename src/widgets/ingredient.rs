use std::{cell::RefCell, rc::Rc};

use egui::{
    Button, CentralPanel, ComboBox, DragValue, Grid, Response, SidePanel, TopBottomPanel, Ui, Widget, special_emojis
};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{ingredient::{Ingredient, IngredientTag, Quality}, ingredient_store::IngredientStore, store::Store, widgets::{create_vec::CreateVecWidget, create_vec_kernels::{VecEnumWidget, VecWidget}}};

pub struct IngredientWidget {
    ingredient_store: Rc<RefCell<IngredientStore>>,
    selected_ingredient: uuid::Uuid,
    tag_editor: CreateVecWidget<String, VecWidget>,
    editing: bool,
}

impl IngredientWidget {
    pub fn new(store: Rc<RefCell<IngredientStore>>) -> IngredientWidget {
        IngredientWidget {
            ingredient_store: store,
            selected_ingredient: uuid::Uuid::nil(),
            tag_editor: CreateVecWidget::default(),
            editing: false
        }
    }

    pub fn show_list(&mut self, ui: &mut egui::Ui, is_liquor: bool) -> Response {
        Grid::new(("ingredient_widget_sidepane_list", is_liquor))
            .striped(true)
            .show(ui, |ui| {
                let mut binding = self.ingredient_store.borrow_mut();
                let mut entries: Vec<(Uuid, &mut Ingredient)> = binding.get_ingredient_entries().into_iter().filter(|i| i.1.is_liquor == is_liquor).collect();
                entries.sort_by_key(|e| e.1.name.to_lowercase());
                for (id, entry) in entries {
                    ui.selectable_value(
                        &mut self.selected_ingredient,
                        id,
                        &entry.name,
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
            }).response
    }
}

impl Widget for &mut IngredientWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        SidePanel::left("ingredient_side_panel_list").show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Mixers");
                        self.show_list(ui, false)
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.label("Liquor");
                        self.show_list(ui, true)
                    }).response
                }).response
            })
        });
        if self.selected_ingredient != uuid::Uuid::nil() {
            TopBottomPanel::bottom("ingredient_footer").show_inside(ui, |ui| {
                let mut store = self.ingredient_store.borrow_mut();
                ui.horizontal(|ui| {
                    if ui.button("Delete entry").clicked() {
                        store.deregister(self.selected_ingredient);
                        self.selected_ingredient = uuid::Uuid::nil();
                    }
                    if ui.toggle_value(&mut self.editing, "Edit ingredient").clicked() {
                        let tags = store.get_tags();
                        if let Some(ingredient) = store.get_entry_mut(self.selected_ingredient) {
                            if self.editing {
                                self.tag_editor = CreateVecWidget::from(
                                    VecWidget::default(),
                                    ingredient.tags.iter().map(|t| t.value.clone()).collect())
                            } else {
                                ingredient.tags = self.tag_editor.get_entries().into_iter().map(|t| IngredientTag { value: t.clone() }).collect();
                            }
                        }
                    }
                })
            });
        }
        CentralPanel::default()
            .show_inside(ui, |ui| {
                if let Some(ingredient) = self
                    .ingredient_store
                    .borrow_mut()
                    .get_entry_mut(self.selected_ingredient)
                {
                    ui.vertical(|ui| {
                        if self.editing {
                            ui.text_edit_singleline(&mut ingredient.name);
                            ui.separator();
                            ui.label("Quality");
                            ComboBox::from_id_salt("ingredient_edit_quality")
                                .selected_text(ingredient.quality.to_string())
                                .show_ui(ui, |ui| {
                                    for quality in Quality::iter() {
                                        ui.selectable_value(
                                            &mut ingredient.quality,
                                            quality,
                                            quality.to_string(),
                                        );
                                    }
                                });
                            ui.separator();
                            ui.add(&mut self.tag_editor);
                        } else {
                            ui.heading(&ingredient.name);
                            ui.separator();
                            ui.label(format!("Quality: {}", ingredient.quality.to_string()));
                            ui.separator();
                            for tag in &ingredient.tags {
                                ui.label(&tag.value);
                            }
                        }
                    });
                }
            })
            .response
    }
}
