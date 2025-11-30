use std::{cell::RefCell, rc::Rc};

use egui::Widget;

use crate::{recipie_builder::RecipieBuilder, recipie_store::{self, RecipieStore}};

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
        let r = ui.text_edit_singleline(&mut self.builder.name);
        if ui.button("Save").clicked() {
            self.recipie_store.borrow_mut().register_recipie(self.builder.build());
        }
        r
    }
}