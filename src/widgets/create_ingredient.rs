use egui::Widget;

#[derive(Default, Clone, Copy)]
pub struct  CreateIngredientWidget {

}

impl Widget for &mut CreateIngredientWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.label("CreateIngredientWidget")
    }
}