use egui::{Align, Layout, Ui, Vec2, Button, RichText, CornerRadius};
use crate::state::app_state::AppView;
use crate::utils::constants::{DIS_VIEW_DASHBOARD, DIS_VIEW_EXPLORER, DIS_VIEW_SETTINGS, DIS_VIEW_TRANSFERT};
pub struct Sidebar;


impl Sidebar {
    pub fn show(ui: &mut Ui, selected: &mut AppView) {
        // Centrage vertical
        ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
            ui.add_space(20.0);

            fn menu_button(ui: &mut Ui, label: &str, is_selected: bool) -> bool {
                let mut btn = Button::new(RichText::new(label).strong())
                    .min_size(Vec2::new(150.0, 20.0))
                    .corner_radius(CornerRadius::same(10));

                if is_selected {
                    btn = btn.fill(ui.visuals().selection.bg_fill);
                }

                ui.add(btn).clicked()
            }

            if menu_button(ui, DIS_VIEW_DASHBOARD, *selected == AppView::Dashboard) {
                *selected = AppView::Dashboard;
            }

            if menu_button(ui, DIS_VIEW_EXPLORER, *selected == AppView::Explorer) {
                *selected = AppView::Explorer;
            }

            if menu_button(ui, DIS_VIEW_TRANSFERT, *selected == AppView::Transfert) {
                *selected = AppView::Transfert;
            }

            if menu_button(ui, DIS_VIEW_SETTINGS, *selected == AppView::Settings) {
                *selected = AppView::Settings;
            }

            ui.add_space(20.0); // Marge basse
        });
    }
}