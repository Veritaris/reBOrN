use crate::app::App;
use egui::Ui;

pub fn debug_stages(ui: &mut Ui, app: &mut App) {
    ui.horizontal(|ui| {
        ui.selectable_value(
            &mut *app.state.lock().unwrap(),
            crate::app::AppState::Started,
            "Started",
        );
        ui.selectable_value(
            &mut *app.state.lock().unwrap(),
            crate::app::AppState::ValidatingCache,
            "ValidatingCache",
        );
        ui.selectable_value(&mut *app.state.lock().unwrap(), crate::app::AppState::Ready, "Ready");
        ui.selectable_value(
            &mut *app.state.lock().unwrap(),
            crate::app::AppState::DownloadingMappings,
            "DownloadingMappings",
        );
        ui.selectable_value(
            &mut *app.state.lock().unwrap(),
            crate::app::AppState::Deobfuscating,
            "Deobfuscating",
        );
    });
}
