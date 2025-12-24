use std::{cell::RefCell, rc::Rc};

use egui::{Button, ComboBox, Layout, ScrollArea, Widget};
use strum::IntoEnumIterator;

use crate::{builder::Builder, ingredient::Quality, ingredient_builder::IngredientBuilder, ingredient_store::IngredientStore, store::Store, widgets::create_vec::{CreateVecWidget, CreateVecWidgetKernel}};

#[derive(Default, Clone)]
pub struct VecWidget {
    entries: Vec<String>
}

impl CreateVecWidgetKernel<String> for VecWidget {
    fn get_entries_mut(&mut self) -> &mut Vec<String> {
        &mut self.entries
    }

    fn get_entry_constraint(&self) -> super::create_vec::EntryConstraint<String> {
        super::create_vec::EntryConstraint::Freeform
    }
    
    fn get_entries(&self) -> Vec<String> {
        self.entries.clone()
    }
}

#[derive(Default, Clone)]
pub struct VecEnumWidget {
    enums: Vec<String>,
    entries: Vec<String>
}

impl VecEnumWidget {
    pub fn new(enums: Vec<String>) -> VecEnumWidget {
        VecEnumWidget { enums: enums, entries: vec![] }
    }
}

impl CreateVecWidgetKernel<String> for VecEnumWidget {
    fn get_entries(&self) -> Vec<String> {
        self.entries.clone()
    }

    fn get_entries_mut(&mut self) -> &mut Vec<String> {
        &mut self.entries
    }

    fn get_entry_constraint(&self) -> super::create_vec::EntryConstraint<String> {
        super::create_vec::EntryConstraint::Enumerated(self.enums.clone())
    }
}

#[derive(Default, Clone)]
pub struct  CreateIngredientWidget {
    builder: IngredientBuilder,
    tag_widget: CreateVecWidget<String, VecWidget>,
    store: Rc<RefCell<IngredientStore>>
}

impl CreateIngredientWidget {
    pub fn new(store: Rc<RefCell<IngredientStore>>) -> Self {
        CreateIngredientWidget {
            builder: IngredientBuilder::default(),
            store: store,
            tag_widget: CreateVecWidget::default(), }
    }

    fn clear(&mut self) {
        self.builder.clear();
        self.tag_widget = CreateVecWidget::default();
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
        ui.separator();
        ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.add(&mut self.tag_widget)
            }).response
        });
        ui.separator();
        ui.horizontal(|ui| {
            let btn = Button::new("Save");
            if ui.add_enabled(!self.builder.name.is_empty(), btn).clicked() {
                self.builder.tags = self.tag_widget.get_entries();
                self.store.borrow_mut().build_from(&self.builder);
                self.clear();
            };
            if ui.button("Reset").clicked() {
                self.clear();
            }
        }).response
    }
}