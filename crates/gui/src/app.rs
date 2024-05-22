use egui::{Style, ViewportCommand, Visuals};
use crate::localisation::localize;


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[allow(non_camel_case_types)]
pub struct App {
    pub cache_web_files: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cache_web_files: true
        }
    }
}

impl App {
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

    pub fn run() -> eframe::Result<()> {
        env_logger::init();
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder {
                title: Some("reBOrN".to_string()),
                decorations: Some(false),
                transparent: Some(true),
                inner_size: Some(egui::vec2(640.0, 480.0)),
                ..Default::default()
            },
            centered: true,
            ..Default::default()
        };
        eframe::run_native(
            "reBOrN",
            options,
            Box::new(|_| {
                Box::<App>::default()
            }),
        )
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        window_frame(ctx, "reBOrN", |ui| {
            ui.label(localize("deobfuscation.input_files"));
            let extra_mappings: Vec<&str> = vec![];
            ui.label(localize("deobfuscation.extra_mappings"));
            if extra_mappings.len() > 0 {
                for one_extra_mapping in extra_mappings {
                    egui::CentralPanel::default().show(ctx, |ui| {});
                }
            };
            // toggle_ui()
            ui.checkbox(&mut self.cache_web_files, localize("deobfuscation.cache_web_files"))
                .on_hover_text(localize("deobfuscation.cache_web_files.tooltip"));
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
}

fn window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(),
        ..Default::default()
    };

    egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, title_bar_rect, title);

        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }.shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(ui: &mut egui::Ui, title_bar_rect: eframe::epaint::Rect, title: &str) {
    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, egui::Id::new("title_bar"), egui::Sense::click());

    painter.text(
        title_bar_rect.center(),
        egui::Align2::CENTER_CENTER,
        title,
        egui::FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    painter.line_segment(
        [
            title_bar_rect.left_bottom() + egui::vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + egui::vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    if title_bar_response.double_clicked() {
        let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(!is_maximized));
    }

    if title_bar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui);
        });
    });
}

fn close_maximize_minimize(ui: &mut egui::Ui) {
    let button_height = 16.0;
    let close_response = ui
        .add(egui::Button::new(egui::RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }

    let is_maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if is_maximized {
        let maximized_response = ui
            .add(egui::Button::new(egui::RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked() {
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(false));
        }
    } else {
        let maximized_response = ui
            .add(egui::Button::new(egui::RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            ui.ctx().send_viewport_cmd(ViewportCommand::Maximized(true));
        }
    }

    let minimized_response = ui
        .add(egui::Button::new(egui::RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
    egui::global_dark_light_mode_switch(ui);
}