use egui::{CentralPanel, ScrollArea, SidePanel, TopBottomPanel, Widget, style::ScrollAnimation};

use std::{cell::RefCell, rc::Rc};

use crate::{
    ingredient_store::IngredientStore,
    recipie::{self, Component},
    recipie_store::RecipieStore,
    store::Store, widgets::{create_component::CreateComponentWidget, create_vec::{CreateVecWidget, CreateVecWidgetKernel}, create_vec_kernels::VecWidget},
};

pub struct RecipieWidget {
    recipie_store: Rc<RefCell<RecipieStore>>,
    ingredient_store: Rc<RefCell<IngredientStore>>,
    selected_recipie: uuid::Uuid,
    old_selected_recipie: uuid::Uuid,
    component_widgets: Vec<ComponentWidget>,
    editing: bool,
    // Only show recipies we can make with our current stock
    show_in_stock: bool,
    edit_instruction_widget: CreateVecWidget<String, VecWidget>,
    edit_components_widget: CreateComponentWidget
}

impl RecipieWidget {
    pub fn new(
        recipie_store: Rc<RefCell<RecipieStore>>,
        ingredient_store: Rc<RefCell<IngredientStore>>,
    ) -> RecipieWidget {
        RecipieWidget {
            recipie_store: recipie_store,
            ingredient_store: ingredient_store.clone(),
            selected_recipie: uuid::Uuid::nil(),
            old_selected_recipie: uuid::Uuid::nil(),
            component_widgets: vec![],
            editing: false,
            show_in_stock: false,
            edit_instruction_widget: CreateVecWidget::default(),
            edit_components_widget: CreateComponentWidget::new(ingredient_store.clone())
        }
    }

    pub fn handle_selection(&mut self) {
        if self.selected_recipie == self.old_selected_recipie {
            return;
        }
        self.old_selected_recipie = self.selected_recipie;
        self.component_widgets.clear();
        if let Some(recipie) = self.recipie_store.borrow().get_entry(self.selected_recipie) {
            self.component_widgets = recipie
                .components
                .iter()
                .map(|c| return ComponentWidget::new(c.clone(), self.ingredient_store.clone()))
                .collect();
        }
    }
}

impl Widget for &mut RecipieWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        SidePanel::left("recipie_side_panel_recipie_list").show_inside(ui, |ui| {
            if ui
                .checkbox(&mut self.show_in_stock, "Show in stock")
                .clicked()
            {
                self.selected_recipie = uuid::Uuid::new_v4();
                let stock = if self.show_in_stock { Some(true) } else { None };
                for entry in self.recipie_store.borrow_mut().get_entries_mut() {
                    entry
                        .components
                        .iter_mut()
                        .for_each(|c| c.ingredient.in_stock = stock);
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
                                ui.selectable_value(&mut self.selected_recipie, id, recipie.name)
                                    .on_hover_text(recipie.short_description);
                            }
                        }
                    } else {
                        for (id, recipie) in recipies {
                            ui.selectable_value(&mut self.selected_recipie, id, recipie.name)
                                .on_hover_text(recipie.short_description);
                        }
                    }
                })
            });
            self.handle_selection();
        });
        if self.selected_recipie != uuid::Uuid::nil() {
            TopBottomPanel::bottom(("recipie_bottom_panel")).show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Delete entry").clicked() {
                        self.recipie_store.borrow_mut().deregister(self.selected_recipie);
                        self.selected_recipie = uuid::Uuid::nil();
                    }
                    if ui.toggle_value(&mut self.editing, "Edit recipie").clicked() {
                        if self.editing && let Some(recipie) = self.recipie_store.borrow_mut().get_entry_mut(self.selected_recipie) {
                            self.edit_instruction_widget = CreateVecWidget::from(VecWidget::default(), recipie.instructions.clone());
                            self.edit_components_widget.set_components(recipie.components.clone());
                        } else if let Some(recipie) = self.recipie_store.borrow_mut().get_entry_mut(self.selected_recipie) {
                            recipie.instructions = self.edit_instruction_widget.get_entries();
                            recipie.components = self.edit_components_widget.get_components();
                            self.component_widgets = recipie
                                .components
                                .iter()
                                .map(|c| return ComponentWidget::new(c.clone(), self.ingredient_store.clone()))
                                .collect();
                        }
                    }
                })
            });
        }
        CentralPanel::default()
            .show_inside(ui, |ui| {
                if let Some(recipie) = self.recipie_store.borrow_mut().get_entry_mut(self.selected_recipie)
                {
                    ScrollArea::vertical().show(ui, |ui| {
                        ui.vertical(|ui| {
                            if self.editing {
                                ui.label("Name:");
                                ui.text_edit_singleline(&mut recipie.name);
                                ui.label("Short description:");
                                ui.text_edit_singleline(&mut recipie.short_description);
                                ui.separator();
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.add(&mut self.edit_instruction_widget);
                                    });
                                    ui.separator();
                                    ScrollArea::vertical().show(ui, |ui| {
                                        ui.add(&mut self.edit_components_widget)
                                    })
                                });
                                ui.separator();
                                ui.label("Description:");
                                ui.text_edit_multiline(&mut recipie.description);
                                ui.separator();
                                ui.label("Notes:");
                                ui.text_edit_multiline(&mut recipie.notes);
                            } else {
                                ui.heading(&recipie.name);
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
                                    ui.label(&recipie.description);
                                }
                                if !recipie.notes.is_empty() {
                                    ui.separator();
                                    ui.label("Notes:");
                                    ui.label(&recipie.notes);
                                }
                            }
                        });
                    }).inner
                }
            })
            .response
    }
}

pub struct ComponentWidget {
    component: Component,
    ingredient_store: Rc<RefCell<IngredientStore>>,
    id: uuid::Uuid,
    selected: String,
}

impl ComponentWidget {
    pub fn new(component: Component, store: Rc<RefCell<IngredientStore>>) -> ComponentWidget {
        let ingredients = store.borrow().select(&component.ingredient);
        let selected = match ingredients.first() {
            Some(val) => val.name.clone(),
            None => String::new(),
        };
        ComponentWidget {
            component: component,
            ingredient_store: store,
            id: uuid::Uuid::new_v4(),
            selected: selected,
        }
    }
}

impl Widget for &mut ComponentWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            let ingredients = self
                .ingredient_store
                .borrow()
                .select(&self.component.ingredient);
            if ingredients.len() == 1 {
                ui.label(ingredients[0].name.clone());
            } else {
                egui::containers::ComboBox::from_id_salt(self.id)
                    .selected_text(self.selected.clone())
                    .show_ui(ui, |ui| {
                        let mut ingredients = self
                            .ingredient_store
                            .borrow()
                            .select(&self.component.ingredient);
                        ingredients.sort_by_key(|i| i.name.clone().to_ascii_lowercase());
                        for ingredient in ingredients {
                            ui.selectable_value(
                                &mut self.selected,
                                ingredient.name.clone(),
                                ingredient.name,
                            );
                        }
                    });
            }
            ui.label(format!(" {}", self.component.amount));
        })
        .response
    }
}
