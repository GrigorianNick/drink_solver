use egui::{Button, CentralPanel, SidePanel, Widget};

use std::{cell::RefCell, rc::Rc};

use crate::{ingredient_store::IngredientStore, recipie::{self, Component}, recipie_store::RecipieStore, store::Store};

pub struct RecipieWidget {
    recipie_store: Rc<RefCell<RecipieStore>>,
    ingredient_store: Rc<RefCell<IngredientStore>>,
    selected_recipie: uuid::Uuid,
    old_selected_recipie: uuid::Uuid,
    component_widgets: Vec<ComponentWidget>,
    editing: bool,
    // Only show recipies we can make with our current stock
    show_in_stock: bool

}

impl RecipieWidget {
    pub fn new(recipie_store: Rc<RefCell<RecipieStore>>, ingredient_store: Rc<RefCell<IngredientStore>>) -> RecipieWidget {
        RecipieWidget { recipie_store: recipie_store, ingredient_store: ingredient_store, selected_recipie: uuid::Uuid::nil(), old_selected_recipie: uuid::Uuid::nil(), component_widgets: vec![], editing: false, show_in_stock: false }
    }

    pub fn handle_selection(&mut self) {
        if self.selected_recipie == self.old_selected_recipie {
            return;
        }
        self.old_selected_recipie = self.selected_recipie;
        self.component_widgets.clear();
        if let Some(recipie) = self.recipie_store.borrow().get_entry(self.selected_recipie) {
            self.component_widgets = recipie.components.iter().map(|c| return ComponentWidget::new(c.clone(), self.ingredient_store.clone())).collect();
        }
    }
}

impl Widget for &mut RecipieWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        SidePanel::left("recipie_side_panel_recipie_list").show_inside(ui, |ui| {
            if ui.checkbox(&mut self.show_in_stock, "Show in stock").clicked() {
                self.selected_recipie = uuid::Uuid::new_v4();
                let stock = if self.show_in_stock { Some(true) } else { None };
                for entry in self.recipie_store.borrow_mut().get_entries_mut() {
                    entry.components.iter_mut().for_each(|c| c.ingredient.in_stock = stock);
                }
            }
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    let mut recipies = self.recipie_store.borrow().get_recipie_entries();
                    recipies.sort_by_key(|r| r.1.name.clone().to_ascii_lowercase());
                    if self.show_in_stock {
                        for (id, recipie) in recipies {
                            if recipie.can_make(self.ingredient_store.clone()) {
                                ui.selectable_value(&mut self.selected_recipie, id, recipie.name).on_hover_text(recipie.short_description);
                            }
                        }
                    } else {
                        for (id, recipie) in recipies {
                            ui.selectable_value(&mut self.selected_recipie, id, recipie.name).on_hover_text(recipie.short_description);
                        }
                    }
                })
            });
            self.handle_selection();
        });
        CentralPanel::default().show_inside(ui, |ui| {
            if let Some(recipie) = self.recipie_store.borrow().get_entry(self.selected_recipie) {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.heading(recipie.name);
                        /*if ui.add(Button::selectable(self.editing, "Edit")).clicked() {
                            self.editing = !self.editing;
                        }*/
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            for (i, step) in recipie.instructions.iter().enumerate() {
                                ui.label(format!("{}. {}", i + 1, step));
                            }
                        });
                        ui.separator();
                        ui.vertical(|ui| {
                            for widget in &mut self.component_widgets {
                                ui.add(widget);
                            }
                        });
                    });
                    if !recipie.description.is_empty() {
                        ui.separator();
                        ui.label("Description:");
                        ui.label(recipie.description);
                    }
                    if !recipie.notes.is_empty() {
                        ui.separator();
                        ui.label("Notes:");
                        ui.label(recipie.notes);
                    }
                });
            }
        }).response
    }
}

pub struct ComponentWidget {
    component: Component,
    ingredient_store: Rc<RefCell<IngredientStore>>, 
    id: uuid::Uuid,
    selected: String
}

impl ComponentWidget {
    pub fn new(component: Component, store: Rc<RefCell<IngredientStore>>) -> ComponentWidget {
        let ingredients = store.borrow().select(&component.ingredient);
        let selected = match ingredients.first() {
            Some(val) => val.name.clone(),
            None => String::new()
        };
        ComponentWidget { component: component, ingredient_store: store, id: uuid::Uuid::new_v4(), selected: selected }
    }
}

impl Widget for &mut ComponentWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            let ingredients = self.ingredient_store.borrow().select(&self.component.ingredient);
            if ingredients.len() == 1 {
                ui.label(ingredients[0].name.clone());
            } else {
                egui::containers::ComboBox::from_id_salt(self.id).selected_text(self.selected.clone()).show_ui(ui, |ui| {
                    let mut ingredients = self.ingredient_store.borrow().select(&self.component.ingredient);
                    ingredients.sort_by_key(|i| i.name.clone().to_ascii_lowercase());
                    for ingredient in ingredients {
                        ui.selectable_value(&mut self.selected, ingredient.name.clone(), ingredient.name);
                    }
                });
            }
            ui.label(format!(" {}", self.component.amount));
        }).response
    }
}