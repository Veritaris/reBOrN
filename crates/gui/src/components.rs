use eframe::epaint::Color32;
use egui::Stroke;
use crate::colors::{Color, HexColor};

// -fx-form-text-color: #f0f0f0;
// -fx-dreamfinity-dark-purple: #463f60;
// -fx-dreamfinity-light-purple: #635093;
//
// -fx-dreamfinity-dialog-background: #24242a;
// -fx-dreamfinity-dark-gray-background: #29292f;
// -fx-dreamfinity-light-gray-background: #303136;
//
// -fx-dreamfinity-action-button-background: #3a3b40;
// -fx-dreamfinity-positive-action-button-border: #31674a;
// -fx-dreamfinity-negative-action-button-border: #6e3035;


const START_COLOR: HexColor = HexColor { hex: "#303136" };
const END_COLOR: HexColor = HexColor { hex: "#635093" };
const TRANSITION_COLOR: HexColor = HexColor { hex: "#635093" };

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
        let col: Color32 = (Color::from(START_COLOR) + (Color::from(END_COLOR) - Color::from(START_COLOR)) * how_on).into();

        ui.painter()
            .rect(
                rect,
                radius,
                col,
                Stroke::NONE,
            );
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(
                center,
                0.75 * radius,
                Color32::from_hex("#f0f0f0").unwrap(),
                Stroke::NONE,
            );
    }

    response
}