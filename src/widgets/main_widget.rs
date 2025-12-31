use std::{cell::RefCell, rc::Rc};

use egui::{CentralPanel, MenuBar, TopBottomPanel};

use super::recipie::RecipieWidget;
use crate::{
    ingredient_store::IngredientStore,
    recipie_store::RecipieStore,
    store::Store,
    widgets::{
        create_ingredient::CreateIngredientWidget, create_recipie::CreateRecipieWidget,
        ingredient::IngredientWidget, styling,
    },
};

#[derive(PartialEq, Eq)]
enum ActiveTab {
    Inventory,
    CreateIngredient,
    Recipies,
    CreateRecipie,
}

pub struct MyApp {
    ingredient_store: Rc<RefCell<IngredientStore>>,
    recipie_store: Rc<RefCell<RecipieStore>>,
    recipie_widget: RecipieWidget,
    ingredient_widget: IngredientWidget,
    create_ingredient_widget: CreateIngredientWidget,
    create_recipie_widget: CreateRecipieWidget,
    active_tab: ActiveTab,
}

impl Default for MyApp {
    fn default() -> Self {
        let recipie_store = Rc::new(RefCell::new(RecipieStore::new()));
        let ingredient_store = Rc::new(RefCell::new(IngredientStore::new()));
        Self {
            ingredient_store: ingredient_store.clone(),
            recipie_store: recipie_store.clone(),
            recipie_widget: RecipieWidget::new(recipie_store.clone(), ingredient_store.clone()),
            ingredient_widget: IngredientWidget::new(ingredient_store.clone()),
            create_ingredient_widget: CreateIngredientWidget::new(ingredient_store.clone()),
            create_recipie_widget: CreateRecipieWidget::new(
                recipie_store.clone(),
                ingredient_store.clone(),
            ),
            active_tab: ActiveTab::Inventory,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("main_header").show(ctx, |ui| {
            ui.heading("Drink Solver");
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Preferences", |ui| styling::build_menu(ctx, ui))
            });
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, ActiveTab::Inventory, "Inventory");
                ui.selectable_value(&mut self.active_tab, ActiveTab::Recipies, "Recipies");
                ui.selectable_value(
                    &mut self.active_tab,
                    ActiveTab::CreateIngredient,
                    "Create Ingredient",
                );
                ui.selectable_value(
                    &mut self.active_tab,
                    ActiveTab::CreateRecipie,
                    "Create Recipie",
                );
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| match self.active_tab {
                ActiveTab::Recipies => ui.add(&mut self.recipie_widget),
                ActiveTab::Inventory => ui.add(&mut self.ingredient_widget),
                ActiveTab::CreateIngredient => ui.add(&mut self.create_ingredient_widget),
                ActiveTab::CreateRecipie => ui.add(&mut self.create_recipie_widget),
            })
            .response
        });
    }
}

impl Drop for MyApp {
    fn drop(&mut self) {
        self.recipie_store.borrow().save();
        self.ingredient_store.borrow().save();
    }
}
