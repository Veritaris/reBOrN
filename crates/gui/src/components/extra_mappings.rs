use crate::app::App;
use crate::localisation::localize;

pub fn extra_mappings_editor(ui: &mut egui::Ui, app: &mut App) {
    egui::CollapsingHeader::new(localize("deobfuscation.extra_mappings_editor"))
        .default_open(false)
        .show_unindented(ui, |ui| {
            ui.label(egui::RichText::new(localize("deobfuscation.extra_mappings")));

            egui::ScrollArea::horizontal()
                .max_height(20.0)
                .min_scrolled_height(20.0)
                .show(ui, |ui| {
                    egui::Grid::new("extra_mappings")
                        .num_columns(3)
                        .spacing([1.0, 1.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("srg");
                            ui.label("mcp");
                            ui.end_row();
                            let len = app.extra_mappings_keys.len();
                            for i in 0..len {
                                match app.extra_mappings_keys.get_mut(i) {
                                    None => continue,
                                    Some(key) => {
                                        ui.add_sized([120.0, 16.0], egui::TextEdit::singleline(key).code_editor());
                                        ui.add_sized(
                                            [240.0, 16.0],
                                            egui::TextEdit::singleline(app.extra_mappings_values.get_mut(i).unwrap())
                                                .code_editor(),
                                        );
                                        if ui.add(egui::Button::new("remove")).clicked() {
                                            app.extra_mappings_keys.remove(i);
                                            app.extra_mappings_values.remove(i);
                                        };
                                        ui.end_row();
                                    }
                                }
                            }
                        });
                });

            if ui.add(egui::Button::new("add mapping")).clicked() {
                app.extra_mappings_keys.push("".to_string());
                app.extra_mappings_values.push("".to_string());
            }
        });
}
