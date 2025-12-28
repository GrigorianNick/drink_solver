use std::{cell::RefCell, rc::Rc};

use egui::{Button, ComboBox, Widget};
use strum::IntoEnumIterator;

use crate::{builder::Builder, component_builder::ComponentBuilder, ingredient::{IngredientTag, Quality, QualityIter}, ingredient_selector_builder::IngredientSelectorBuilder, ingredient_store::{IngredientSelector, IngredientStore}, recipie::Component, widgets::{create_ingredient::{VecEnumWidget, VecWidget}, create_vec::{CreateVecWidget, CreateVecWidgetKernel}}};

#[derive(Clone, Default)]
pub struct CreateComponentEntryWidget {
    builder: ComponentBuilder,
    store: Rc<RefCell<IngredientStore>>,
    tag_widget: CreateVecWidget<String, VecEnumWidget>,
    id: uuid::Uuid
}

impl CreateComponentEntryWidget {
    pub fn new(store: Rc<RefCell<IngredientStore>>) -> CreateComponentEntryWidget {
        let tags = store.borrow().get_tags().iter().map(|t| t.value.clone()).collect();
        CreateComponentEntryWidget {
            builder: ComponentBuilder::default(),
            store,
            tag_widget: CreateVecWidget::new(VecEnumWidget::new(tags)),
            id: uuid::Uuid::new_v4() }
    }

    pub fn build(&self) -> Component {
        let mut component = self.builder.build();
        let tag_vals = self.tag_widget.get_entries();
        if !tag_vals.is_empty() {
            let tags = tag_vals.into_iter().map(|t| IngredientTag{ value: t.clone()}).collect();
            component.ingredient.tags = Some(tags)
        }
        component
    }
}

impl Widget for &mut CreateComponentEntryWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.builder.selector.name);
                ui.label("Quality");
                ComboBox::from_id_salt(self.id)
                    .selected_text(self.builder.selector.quality.unwrap_or_default().to_string())
                    .show_ui(ui, |ui| {
                        for quality in Quality::iter() {
                            ui.selectable_value(&mut self.builder.selector.quality, Some(quality), quality.to_string());
                        }
                })
            });
            ui.separator();
            ui.add(&mut self.tag_widget)
        }).response
    }
}

#[derive(Clone)]
pub struct CreateComponentWidget {
    entries: Vec<CreateComponentEntryWidget>,
    builders: Vec<ComponentBuilder>,
    store: Rc<RefCell<IngredientStore>>
}

impl CreateComponentWidget {

    pub fn new(store: Rc<RefCell<IngredientStore>>) -> CreateComponentWidget {
        CreateComponentWidget { entries: vec![], builders: vec![], store: store }
    }

    pub fn get_components(&self) -> Vec<Component> {
        self.entries.iter().map(|entry| entry.build()).collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear()
    }
}

impl Widget for &mut CreateComponentWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut names = self.store.borrow().get_ingredient_names();
        names.sort_by_key(|s| s.to_lowercase());
        let mut tags = self.store.borrow().get_tags();
        tags.sort_by_key(|t| t.value.to_lowercase());
        egui::ScrollArea::vertical().show(ui, |ui|{
            ui.vertical(|ui| {
                for entry in &mut self.entries {
                    ui.add(entry);
                    ui.separator();
                }
                let resp = ui.button("Add Component");
                if resp.clicked() {
                    self.entries.push(CreateComponentEntryWidget::new(self.store.clone()));
                }
                resp
            }).response
        }).inner
    }
}