use eframe::epaint::Color32;

pub fn picked_files<Consumer>(
    ui: &mut egui::Ui,
    files: &[std::path::PathBuf],
    file_idx: Option<usize>,
    mut on_remove: Consumer,
) -> egui::Response
where
    Consumer: FnMut(&std::path::PathBuf, usize),
{
    let (rect, response) = ui.allocate_exact_size(egui::Vec2::ZERO, egui::Sense::click());
    let file_idx = file_idx.unwrap_or(0);
    let max_chip_width = ui.available_width().max(80.0);
    egui::Frame::NONE
        .fill(Color32::TRANSPARENT)
        .inner_margin(egui::Margin::symmetric(6, 0))
        .corner_radius(egui::CornerRadius::same(10))
        .stroke(egui::Stroke::new(0.8, Color32::WHITE))
        .show(ui, |ui| {
            ui.set_max_width(max_chip_width);
            ui.spacing_mut().item_spacing.x = 4.0;
            ui.horizontal(|ui| {
                let file = files.get(file_idx);
                if let Some(file) = file
                    && ui.is_rect_visible(rect)
                {
                    let filename = format!("{}", file.file_name().unwrap_or_default().display());
                    let filename_text = egui::RichText::new(filename).color(Color32::WHITE);
                    ui.add(egui::Label::new(filename_text).truncate());
                    let close_response = ui
                        .add(
                            egui::Button::new(egui::RichText::new("❌"))
                                .fill(Color32::TRANSPARENT)
                                .frame(false),
                        )
                        .on_hover_text("Remove file from list");
                    if close_response.clicked() {
                        on_remove(file, file_idx);
                    }
                }
            });
        });

    response
}
