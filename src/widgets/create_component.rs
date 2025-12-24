use std::{cell::RefCell, rc::Rc};

use egui::{Button, ComboBox, Widget};
use strum::IntoEnumIterator;

use crate::{builder::Builder, component_builder::ComponentBuilder, ingredient::{IngredientTag, Quality, QualityIter}, ingredient_selector_builder::IngredientSelectorBuilder, ingredient_store::{IngredientSelector, IngredientStore}, recipie::Component, widgets::{create_ingredient::VecEnumWidget, create_vec::{CreateVecWidget, CreateVecWidgetKernel}}};

#[derive(Clone, Default)]
pub struct CreateComponentEntryWidget {
    builder: ComponentBuilder,
    store: Rc<RefCell<IngredientStore>>
}

impl CreateComponentEntryWidget {
    pub fn new(store: Rc<RefCell<IngredientStore>>) -> CreateComponentEntryWidget {
        CreateComponentEntryWidget { builder: ComponentBuilder::default(), store }
    }
}

#[derive(Clone)]
pub struct CreateComponentWidget {
    builders: Vec<ComponentBuilder>,
    store: Rc<RefCell<IngredientStore>>
}

impl CreateComponentWidget {

    pub fn new(store: Rc<RefCell<IngredientStore>>) -> CreateComponentWidget {
        CreateComponentWidget { builders: vec![], store: store }
    }

    // Returns response and if it should be deleted or not
    fn add_entry(ui: &mut egui::Ui, idx: usize, builder: &mut ComponentBuilder, names: &Vec<String>, tags: &Vec<IngredientTag>) -> (egui::Response, bool) {
        let mut should_delete = false;
        let resp = ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Name");
                ComboBox::from_id_salt(("Name", idx)).selected_text(builder.selector.name.clone()).show_ui(ui, |ui| {
                    for name in names {
                        ui.selectable_value(&mut builder.selector.name, name.clone(), name.clone());
                    }
                });
                ui.label("Quality");
                ComboBox::from_id_salt(("Quality", idx)).selected_text(builder.selector.quality.unwrap_or_default().to_string()).show_ui(ui, |ui| {
                    for quality in Quality::iter() {
                        ui.selectable_value(&mut builder.selector.quality, Some(quality), quality.to_string());
                    }
                }).response
            });
            ui.separator();
            let ts: Vec<String> = tags.into_iter().map(|t| t.value.clone()).collect();
            ui.add(&mut CreateVecWidget::new(VecEnumWidget::new(ts)));
            ui.separator();
            let resp = ui.add(Button::new("X"));
            if resp.clicked() {
                should_delete = true
            }
            resp
        }).response;
        (resp, should_delete)
    }

    pub fn get_components(&self) -> Vec<Component> {
        self.builders.iter().map(|b| b.build()).collect()
    }
}

impl Widget for &mut CreateComponentWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut names = self.store.borrow().get_ingredient_names();
        names.sort_by_key(|s| s.to_lowercase());
        let mut tags = self.store.borrow().get_tags();
        tags.sort_by_key(|t| t.value.to_lowercase());
        ui.vertical(|ui| {
            let mut to_remove = None;
            for (i, builder) in self.builders.iter_mut().enumerate() {
                if let (_, true) = CreateComponentWidget::add_entry(ui, i, builder, &names, &tags) {
                    to_remove = Some(i)
                }
                ui.separator();
            }
            if let Some(idx) = to_remove {
                self.builders.remove(idx);
            }
            let resp = ui.button("Add entry");
            if resp.clicked() {
                self.builders.push(ComponentBuilder::default());
            }
            resp
        }).response
    }
}