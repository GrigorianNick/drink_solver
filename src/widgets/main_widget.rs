use std::{cell::RefCell, rc::Rc};

use crate::{ingredients::{Ingredient, IngredientSelector, IngredientStore, IngredientTag, Quality}, recipie::{Component, Recipie}, recipie_store::{self, RecipieStore}, widgets::{create_ingredient::CreateIngredientWidget, create_recipie::CreateRecipieWidget, ingredient::{self, IngredientWidget}}};
use super::recipie::RecipieWidget;

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
    active_tab: ActiveTab
}

impl Default for MyApp {
    fn default() -> Self {
        let mut recipie_store = Rc::new(RefCell::new(RecipieStore::default()));
        let mut ingredient_store = Rc::new(RefCell::new(IngredientStore::new()));
        let gin = Ingredient { name: "Roku".into(), quality: crate::ingredients::Quality::High, tags: vec![IngredientTag{ value: "Gin".into()}] };
        let gin2 = Ingredient { name: "Virago".into(), quality: crate::ingredients::Quality::High, tags: vec![IngredientTag{ value: "Gin".into()}] };
        let rum = Ingredient { name: "Kraken".into(), quality: crate::ingredients::Quality::High, tags: vec![IngredientTag{ value: "Rum".into()}] };
        let simple_syrup = Ingredient { name: "Simple Syrup".into(), quality: crate::ingredients::Quality::Any, tags: vec![IngredientTag{ value: "Sweet".into()},IngredientTag{ value: "Sugary".into()}]};
        let tonic = Ingredient { name: "Tonic water".into(), quality: crate::ingredients::Quality::Any, tags: vec![IngredientTag{ value: "Fizzy".into()}]};
        let cola = Ingredient { name: "Coca-Cola".into(), quality: crate::ingredients::Quality::Any, tags: vec![IngredientTag{ value: "Cola".into()}]};
        ingredient_store.borrow_mut().register_ingredient(gin);
        ingredient_store.borrow_mut().register_ingredient(gin2);
        ingredient_store.borrow_mut().register_ingredient(rum);
        ingredient_store.borrow_mut().register_ingredient(simple_syrup);
        ingredient_store.borrow_mut().register_ingredient(tonic);
        ingredient_store.borrow_mut().register_ingredient(cola);

        let gin_and_tonic = Recipie{
            name: "Gin and Tonic".into(),
            description: "G&T".into(),
            short_description: "Fizzy ouch".into(),
            notes: "This cocktail is kind of painful to drink".into(),
            components: vec![
                Component{
                    ingredient: IngredientSelector{ name: None, quality: None, tags: Some(vec![IngredientTag{ value: "Gin".into()}]) },
                    amount: crate::recipie::Measure::Oz(2.5)
                },
                Component{
                    ingredient: IngredientSelector{ name: Some("Tonic water".into()), quality: None, tags: None },
                    amount: crate::recipie::Measure::Oz(2.5)
                }
                ],
            instructions: vec!["Put in Gin".into(), "Put in tonic".into(), "Mix".into()]
            };
        let rum_and_coke = Recipie{
            name: "Rum and coke".into(),
            description: "R&C".into(),
            short_description: "Fizzy ouch yum".into(),
            notes: "My default goto".into(),
            components: vec![
                Component{
                    ingredient: IngredientSelector{ name: None, quality: None, tags: Some(vec![IngredientTag{ value: "Rum".into()}]) },
                    amount: crate::recipie::Measure::Oz(2.5)
                },
                Component{
                    ingredient: IngredientSelector{ name: None, quality: None, tags: Some(vec![IngredientTag{ value: "Cola".into()}]) },
                    amount: crate::recipie::Measure::Oz(5.0)
                }
                ],
            instructions: vec!["Put in ice".into(), "Put in rum".into(), "Top with coke".into()]
            };

        recipie_store.borrow_mut().register_recipie(gin_and_tonic);
        recipie_store.borrow_mut().register_recipie(rum_and_coke);

        Self {
            ingredient_store: ingredient_store.clone(),
            recipie_store: recipie_store.clone(),
            recipie_widget: RecipieWidget::new(recipie_store.clone(), ingredient_store.clone()),
            ingredient_widget: IngredientWidget::new(ingredient_store),
            create_ingredient_widget: CreateIngredientWidget::default(),
            create_recipie_widget: CreateRecipieWidget::new(recipie_store.clone()),
            active_tab: ActiveTab::Inventory
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Drink Solver");
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Inventory, "Inventory");
                    ui.selectable_value(&mut self.active_tab, ActiveTab::Recipies, "Recipies");
                    ui.selectable_value(&mut self.active_tab, ActiveTab::CreateIngredient, "Create Ingredient");
                    ui.selectable_value(&mut self.active_tab, ActiveTab::CreateRecipie, "Create Recipie");
                });
                ui.separator();
                match self.active_tab {
                    ActiveTab::Recipies => ui.add(&mut self.recipie_widget),
                    ActiveTab::Inventory => ui.add(&mut self.ingredient_widget),
                    ActiveTab::CreateIngredient => ui.add(&mut self.create_ingredient_widget),
                    ActiveTab::CreateRecipie => ui.add(&mut self.create_recipie_widget),
                }
            });
        });
    }
}