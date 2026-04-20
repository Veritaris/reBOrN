use egui::{Style, Visuals};


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct reBOrNApp {}

impl Default for reBOrNApp {
    fn default() -> Self {
        Self {}
    }
}

impl reBOrNApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let style = Style {
            visuals: Visuals::dark(),
            ..Style::default()
        };
        cc.egui_ctx.set_style(style);
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for reBOrNApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
