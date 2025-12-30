use egui::{Color32, Stroke, Style, Theme};

pub fn build_menu(ctx: &egui::Context, ui: &mut egui::Ui) -> egui::Response {
    ui.menu_button("Style", |ui| {
        if ui.button("Dark").clicked() {
            ctx.set_theme(Theme::Dark);
        }
        if ui.button("Light").clicked() {
            ctx.set_theme(Theme::Light);
        }
        if ui.button("Neon").clicked() {
            ctx.style_mut(set_neon);
        }
    }).response
}

fn set_neon(style: &mut Style) {
    style.visuals.button_frame = true;
    // hovering
    style.visuals.widgets.hovered.weak_bg_fill = Color32::YELLOW;
    style.visuals.widgets.hovered.fg_stroke.color = Color32::MAGENTA;

    // inactive (at rest)
    style.visuals.widgets.inactive.fg_stroke.color = Color32::CYAN;
    style.visuals.widgets.inactive.bg_stroke.color = Color32::GREEN;

    // active (doing stuff)
    style.visuals.widgets.active.fg_stroke.color = Color32::MAGENTA;
    style.visuals.widgets.active.weak_bg_fill = Color32::YELLOW;

    style.visuals.widgets.open.fg_stroke.color = Color32::GREEN;
    style.visuals.selection.stroke.color = Color32::GREEN;
    style.visuals.selection.bg_fill = Color32::BLACK;

    style.visuals.window_fill = Color32::TRANSPARENT;
    style.visuals.panel_fill = Color32::TRANSPARENT;
}