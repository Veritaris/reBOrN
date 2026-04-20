use crate::localisation::localize;
use egui::Ui;

pub fn app_validating_cache(ui: &mut Ui) {
    ui.horizontal_centered(|ui| {
        ui.vertical_centered_justified(|ui| {
            ui.label(egui::RichText::new(localize("Downloading versions.json")));
            ui.add(egui::Spinner::new().size(32.0));
        });
    });
}
