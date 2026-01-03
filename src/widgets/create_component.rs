use std::{cell::RefCell, rc::Rc};

use egui::{ComboBox, DragValue, Widget};
use strum::IntoEnumIterator;

use crate::{
    builder::Builder,
    component_builder::ComponentBuilder,
    ingredient::{IngredientTag, Quality},
    ingredient_store::IngredientStore,
    measure::Measure,
    recipie::Component,
    widgets::{create_vec::CreateVecWidget, create_vec_kernels::VecEnumWidget},
};

#[derive(Clone, Default)]
pub struct CreateComponentEntryWidget {
    builder: ComponentBuilder,
    tag_widget: CreateVecWidget<String, VecEnumWidget>,
    id: uuid::Uuid,
    tags: Vec<String>,
    names: Vec<String>
}

impl CreateComponentEntryWidget {
    pub fn new(store: Rc<RefCell<IngredientStore>>) -> CreateComponentEntryWidget {
        let mut tags: Vec<String> = store
            .borrow()
            .get_tags()
            .iter()
            .map(|t| t.value.clone())
            .collect();
        tags.sort_by_key(|t| t.to_ascii_lowercase());
        let mut names = store.borrow().get_ingredient_names();
        names.sort_by_key(|n| n.to_ascii_lowercase());
        CreateComponentEntryWidget {
            builder: ComponentBuilder::default(),
            tag_widget: CreateVecWidget::new(VecEnumWidget::new(tags.clone())),
            id: uuid::Uuid::new_v4(),
            tags: tags,
            names: names
        }
    }

    pub fn set_component(&mut self, component: Component) {
        self.builder = ComponentBuilder::from(component.clone());
        self.tag_widget = CreateVecWidget::from(
            VecEnumWidget::new(self.tags.clone()),
            component.ingredient.tags.unwrap_or_default().into_iter().map(|s| s.value).collect());
    }

    pub fn build(&self) -> Component {
        let mut component = self.builder.build();
        let tag_vals = self.tag_widget.get_entries();
        if !tag_vals.is_empty() {
            let tags = tag_vals
                .into_iter()
                .map(|t| IngredientTag { value: t.clone() })
                .collect();
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
                ComboBox::from_id_salt(("Names", self.id)).selected_text(&self.builder.selector.name).show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.builder.selector.name, String::from(""), "");
                    for name in &self.names {
                        ui.selectable_value(&mut self.builder.selector.name, name.clone(), name);
                    }
                });
                ui.label("Quality");
                ComboBox::from_id_salt(("Quality", self.id))
                    .selected_text(
                        self.builder
                            .selector
                            .quality
                            .unwrap_or_default()
                            .to_string(),
                    )
                    .show_ui(ui, |ui| {
                        for quality in Quality::iter() {
                            ui.selectable_value(
                                &mut self.builder.selector.quality,
                                Some(quality),
                                quality.to_string(),
                            );
                        }
                    });
                ui.label("Quantity");
                ui.horizontal(|ui| {
                    match &mut self.builder.measure {
                        Measure::Taste => (),
                        Measure::Oz(val) => {
                            ui.add(DragValue::new(val).speed(0.1));
                        }
                        Measure::Shot(val) => {
                            ui.add(DragValue::new(val).speed(0.1));
                        }
                        Measure::Ml(val) => {
                            ui.add(DragValue::new(val).speed(0.1));
                        }
                        Measure::Liter(val) => {
                            ui.add(DragValue::new(val).speed(0.1));
                        }
                        Measure::Handle(val) => {
                            ui.add(DragValue::new(val).speed(0.1));
                        }
                    };
                    ComboBox::from_id_salt(("Quantity", self.id))
                        .selected_text(self.builder.measure.to_string())
                        .show_ui(ui, |ui| {
                            for measure in Measure::iter() {
                                ui.selectable_value(
                                    &mut self.builder.measure,
                                    measure.clone(),
                                    measure.to_string(),
                                );
                            }
                        })
                })
            });
            ui.separator();
            ui.vertical(|ui| ui.add(&mut self.tag_widget))
        })
        .response
    }
}

#[derive(Clone)]
pub struct CreateComponentWidget {
    entries: Vec<CreateComponentEntryWidget>,
    store: Rc<RefCell<IngredientStore>>,
}

impl CreateComponentWidget {
    pub fn new(store: Rc<RefCell<IngredientStore>>) -> CreateComponentWidget {
        CreateComponentWidget {
            entries: vec![],
            store: store,
        }
    }

    pub fn get_components(&self) -> Vec<Component> {
        self.entries.iter().map(|entry| entry.build()).collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear()
    }

    pub fn set_components(&mut self, components: Vec<Component>) {
        self.clear();
        for component in components {
            let mut widget = CreateComponentEntryWidget::new(self.store.clone());
            widget.set_component(component);
            self.entries.push(widget);
        }
    }
}

impl Widget for &mut CreateComponentWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut names = self.store.borrow().get_ingredient_names();
        names.sort_by_key(|s| s.to_lowercase());
        let mut tags = self.store.borrow().get_tags();
        tags.sort_by_key(|t| t.value.to_lowercase());
        ui.vertical(|ui| {
            for entry in &mut self.entries {
                ui.add(entry);
                ui.separator();
            }
            let resp = ui.button("Add Component");
            if resp.clicked() {
                self.entries
                    .push(CreateComponentEntryWidget::new(self.store.clone()));
            }
            resp
        })
        .response
    }
}
