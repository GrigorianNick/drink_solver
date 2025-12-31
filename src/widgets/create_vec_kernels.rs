use crate::widgets::create_vec::CreateVecWidgetKernel;


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

    fn clear(&mut self) {
        self.entries.clear();
    }
}

#[derive(Default, Clone)]
pub struct VecEnumWidget {
    enums: Vec<String>,
    entries: Vec<String>,
    id: uuid::Uuid
}

impl VecEnumWidget {
    pub fn new(enums: Vec<String>) -> VecEnumWidget {
        VecEnumWidget { enums: enums, entries: vec![], id: uuid::Uuid::new_v4() }
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
        super::create_vec::EntryConstraint::Enumerated(self.enums.clone(), self.id)
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}