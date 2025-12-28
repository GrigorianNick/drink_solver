use std::{marker::PhantomData, str::FromStr};

use egui::{ComboBox, Widget};


pub enum EntryConstraint<T> {
    Freeform,
    Enumerated(Vec<T>, uuid::Uuid)
}

pub trait CreateVecWidgetKernel<T: Default + egui::TextBuffer> {
    fn clear(&mut self);
    fn get_entries(&self) -> Vec<T>;
    fn get_entries_mut(&mut self) -> &mut Vec<T>;
    fn get_entry_constraint(&self) -> EntryConstraint<T>;
}

#[derive(Default, Clone)]
pub struct CreateVecWidget<T, Kernel> where T: Default + egui::TextBuffer, Kernel: CreateVecWidgetKernel<T> {
    kernel: Kernel,
    phantom: PhantomData<T>
}

impl<T:Default + egui::TextBuffer + PartialEq + Clone, Kernel: CreateVecWidgetKernel<T>> CreateVecWidget<T, Kernel> {
    fn build_freeform(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let mut to_remove = vec![];
        for (i, val) in self.kernel.get_entries_mut().iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(val);
                if ui.button("X").clicked() {
                    to_remove.push(i);
                }
            });
        }
        for i in to_remove.iter().rev() {
            self.kernel.get_entries_mut().remove(*i);
        }
        let b = ui.button("Add entry");
        if b.clicked() {
            self.kernel.get_entries_mut().push(T::default());
        }
        b
    }

    fn build_enumerated(&mut self, ui: &mut egui::Ui, enums: &Vec<T>, id: impl std::hash::Hash) -> egui::Response {
        if enums.is_empty() {
            return ui.label("No options provided!");
        }
        for (i, val) in self.kernel.get_entries_mut().iter_mut().enumerate() {
            ComboBox::from_id_salt((i, &id)).selected_text(val.as_str()).show_ui(ui, |ui| {
                for e in enums {
                    ui.selectable_value(val, e.clone(), e.as_str());
                }
            });
        }
        let b = ui.button("Add entry");
        if b.clicked() {
            self.kernel.get_entries_mut().push(T::default());
        }
        b
    }

    pub fn get_entries(&self) -> Vec<T> {
        self.kernel.get_entries()
    }

    pub fn new(kernel: Kernel) -> CreateVecWidget<T, Kernel> {
        CreateVecWidget { kernel, phantom: PhantomData::default() }
    }

    pub fn clear(&mut self) {
        self.kernel.clear();
    }
}

impl<T:Default + egui::TextBuffer + PartialEq + Clone, Kernel: CreateVecWidgetKernel<T>> Widget for &mut CreateVecWidget<T, Kernel> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        match self.kernel.get_entry_constraint() {
            EntryConstraint::Freeform => self.build_freeform(ui),
            EntryConstraint::Enumerated(items, id) => self.build_enumerated(ui, &items, id),
        }
    }
}