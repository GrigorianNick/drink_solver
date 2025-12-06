use std::{marker::PhantomData, str::FromStr};

use egui::{ComboBox, Widget};

// A dumb because trait aliasing is stuck in unstable limbo
pub trait VecType: Default + ToString + FromStr{}

pub enum EntryConstraint<T> {
    Freeform,
    Enumerated(Vec<T>)
}

pub trait CreateVecWidgetKernel<T: VecType> {
    fn get_entries_mut(&self) -> &mut Vec<T>;
    fn get_entry_constraint(&self) -> EntryConstraint<T>;
}

pub struct CreateVecWidget<T, Kernel> where T: VecType, Kernel: CreateVecWidgetKernel<T> {
    kernel: Kernel,
    phantom: PhantomData<T>
}

impl<T:VecType, Kernel: CreateVecWidgetKernel<T>> CreateVecWidget<T, Kernel> {
    fn build_freeform(&self, ui: &mut egui::Ui, val: &mut T) {
        ui.text_edit_singleline(val);
    }

    fn build_enumerated(&self, ui: &mut egui::Ui, val: &mut T) {
        //ComboBox::from_id_salt(id_salt)
    }
}

impl<T:VecType, Kernel: CreateVecWidgetKernel<T>> Widget for &mut CreateVecWidget<T, Kernel> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let vals: &mut Vec<T> = self.kernel.get_entries_mut();
        for (i, val) in vals.iter_mut().enumerate() {
            match self.kernel.get_entry_constraint() {
                EntryConstraint::Freeform => self.build_freeform(ui, val),
                EntryConstraint::Enumerated(items) => self.build_enumerated(ui, val),
            }
        }
        ui.separator()
    }
}