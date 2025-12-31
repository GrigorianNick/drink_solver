use std::{cell::RefCell, rc::Rc};

use egui::{Button, CentralPanel, ComboBox, ScrollArea, TopBottomPanel, Widget};
use strum::IntoEnumIterator;

use crate::{builder::Builder, ingredient::Quality, ingredient_builder::IngredientBuilder, ingredient_store::IngredientStore, store::Store, widgets::{create_vec::{CreateVecWidget, CreateVecWidgetKernel}, create_vec_kernels::VecWidget}};

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
        TopBottomPanel::bottom("create_ingredient_widget").show_inside(ui, |ui| {
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
            })
        });
        CentralPanel::default().show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut self.builder.name)
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.label("Quality");
                    ComboBox::from_id_salt("CreateIngredientWidgetQuaulity")
                        .selected_text(self.builder.quality.to_string())
                        .show_ui(ui, |ui| {
                        for quality in Quality::iter() {
                            ui.selectable_value(&mut self.builder.quality,  quality, quality.to_string());
                        }
                    })
                })
            });
            ui.separator();
            ui.label("Tags");
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.add(&mut self.tag_widget)
                }).response
            })
        }).response
    }
}