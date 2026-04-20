use crate::colors::{Color, HexColor};
use crate::ui_consts::{END_COLOR, START_COLOR};
use eframe::epaint::{Color32, StrokeKind};

#[allow(dead_code)]
pub fn toggle_ui_compact(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, true, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let radius = 0.5 * rect.height();

        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let col: Color32 =
            (Color::from(START_COLOR) + (Color::from(END_COLOR) - Color::from(START_COLOR)) * how_on).into();

        ui.painter()
            .rect(rect, radius, col, egui::Stroke::NONE, StrokeKind::Middle);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter().circle(
            center,
            0.75 * radius,
            Color32::from_hex("#f0f0f0").unwrap(),
            egui::Stroke::NONE,
        );
    }

    response
}
