use crate::state::app_state::AppView;
use crate::ui::{sidebar::Sidebar, dashboard, explorer, settings, transfert};

pub struct MainApp {
    pub view: AppView,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            view: AppView::Dashboard,
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            Sidebar::show(ui, &mut self.view);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.view {
                AppView::Dashboard => dashboard::show(ui),
                AppView::Explorer => explorer::show(ui),
                AppView::Settings => settings::show(ui),
                AppView::Transfert => transfert::show(ui),
            }
        });
    }
}
