use std::{cell::RefCell, rc::Rc};

use egui::Widget;

use crate::{builder::Builder, recipie_builder::RecipieBuilder, recipie_store::{self, RecipieStore}, store::Store};

#[derive(Clone)]
pub struct CreateRecipieWidget {
    builder: RecipieBuilder,
    recipie_store: Rc<RefCell<RecipieStore>>
}

impl CreateRecipieWidget {
    pub fn new(store: Rc<RefCell<RecipieStore>>) -> CreateRecipieWidget {
        CreateRecipieWidget { builder: RecipieBuilder::default(), recipie_store: store.clone() }
    }
}

impl Widget for &mut CreateRecipieWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.text_edit_singleline(&mut self.builder.name);
        ui.separator();
        ui.text_edit_singleline(&mut self.builder.short_description);
        ui.text_edit_multiline(&mut self.builder.description);
        ui.horizontal( |ui| {
            if ui.button("Save").clicked() {
                self.recipie_store.borrow_mut().register(self.builder.build());
            }
            if ui.button("Reset").clicked() {
                self.builder = RecipieBuilder::new();
            }
        }).response
    }
}