use std::{cell::RefCell, rc::Rc};

use egui::{Button, Layout, ScrollArea, Separator, Widget};

use crate::{builder::Builder, ingredient_store::IngredientStore, recipie_builder::RecipieBuilder, recipie_store::{self, RecipieStore}, store::Store, widgets::{create_component::CreateComponentWidget, create_ingredient::VecWidget, create_vec::CreateVecWidget}};

#[derive(Clone)]
pub struct CreateRecipieWidget {
    builder: RecipieBuilder,
    recipie_store: Rc<RefCell<RecipieStore>>,
    component_widget: CreateComponentWidget,
    instruction_widget: CreateVecWidget<String, VecWidget>
}

impl CreateRecipieWidget {
    pub fn new(store: Rc<RefCell<RecipieStore>>, ingredient_store: Rc<RefCell<IngredientStore>>) -> CreateRecipieWidget {
        CreateRecipieWidget {
            builder: RecipieBuilder::default(),
            recipie_store: store.clone(),
            component_widget: CreateComponentWidget::new(ingredient_store.clone()),
            instruction_widget: CreateVecWidget::default() }
    }
}

impl Widget for &mut CreateRecipieWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label("Name");
        ui.text_edit_singleline(&mut self.builder.name);
        ui.separator();
        ui.horizontal( |ui| {
            ui.vertical(|ui| {
                ui.label("Short description");
                ui.text_edit_singleline(&mut self.builder.short_description);
                ui.label("Description");
                ui.text_edit_multiline(&mut self.builder.description);
                ui.label("Notes");
                ui.text_edit_multiline(&mut self.builder.notes)
            });
            ui.separator();
            ui.push_id(0, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label("Instructions");
                        ui.add(&mut self.instruction_widget)
                    })
                })
            });
            ui.separator();
            ui.push_id(1, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.add(&mut self.component_widget)
                })
            })
        });
        ui.separator();
        ui.horizontal( |ui| {
            let btn = Button::new("Save");
            if ui.add_enabled(!self.builder.name.is_empty(), btn).clicked() {
                self.builder.instructions = self.instruction_widget.get_entries();
                self.recipie_store.borrow_mut().build_from(&self.builder);
                self.builder.clear();
            }
            if ui.button("Reset").clicked() {
                self.instruction_widget.clear();
                self.component_widget.clear();
                self.builder.clear()
            }
        }).response
    }
}