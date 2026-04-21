use crate::app::App;
use crate::components;
use crate::localisation::localize;
use mc_deobf::mappings::ModLoader;

pub fn remapper_args_block(ui: &mut egui::Ui, app: &mut App) {
    egui::CollapsingHeader::new(localize("deobfuscation.remapper_args"))
        .default_open(false)
        .show_unindented(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.verbosity")))
                    .on_hover_text(localize("options.verbosity.tooltip"));
                egui::ComboBox::from_id_salt("verbose")
                    .selected_text(format!("{}", &app.verbose))
                    .show_ui(ui, |ui| {
                        for i in 0..=3 {
                            ui.selectable_value(&mut app.verbose, i, format!("{}", i));
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.deobf_suffix")))
                    .on_hover_text(localize("options.deobf_suffix.tooltip"));
                ui.text_edit_singleline(&mut app.deobf_suffix_value);
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.debug")))
                    .on_hover_text(localize("options.debug.tooltip"));
                components::toggle_ui_compact(ui, &mut app.debug);
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.print_class")))
                    .on_hover_text(localize("options.print_class.tooltip"));
                components::toggle_ui_compact(ui, &mut app.print_class);
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.print_cpool")))
                    .on_hover_text(localize("options.print_cpool.tooltip"));
                components::toggle_ui_compact(ui, &mut app.print_cpool);
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.print_code")))
                    .on_hover_text(localize("options.print_code.tooltip"));
                components::toggle_ui_compact(ui, &mut app.print_code);
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.mod_loader")))
                    .on_hover_text(localize("options.mod_loader.tooltip"));
                egui::ComboBox::from_id_salt("mod_loader")
                    .selected_text(format!("{}", &app.mod_loader))
                    .show_ui(ui, |ui| {
                        components::ui_select_enum_case!(ui, app.mod_loader, ModLoader::Forge);
                        components::ui_select_enum_case!(ui, app.mod_loader, ModLoader::NeoForge);
                        components::ui_select_enum_case!(ui, app.mod_loader, ModLoader::Fabric);
                    });
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.show_versions")))
                    .on_hover_text(localize("options.show_versions.tooltip"));
                components::toggle_ui_compact(ui, &mut app.show_versions);
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.cache_dir")))
                    .on_hover_text(localize("options.cache_dir.tooltip"));
                components::toggle_ui_compact(ui, &mut app.cache_dir);
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.set_cache_dir")))
                    .on_hover_text(localize("options.set_cache_dir.tooltip"));

                if ui.button("deobfuscation.select_cache_dir").clicked()
                    && let Some(cache_dir_folder) =
                        rfd::FileDialog::new().set_directory(&app.set_cache_dir).pick_folder()
                    && let Some(cache_dir) = cache_dir_folder.to_str()
                {
                    app.set_cache_dir = cache_dir.to_string();
                }
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.clean_cache_dir")))
                    .on_hover_text(localize("options.clean_cache_dir.tooltip"));
                components::toggle_ui_compact(ui, &mut app.clean_cache_dir);
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.output_dir")))
                    .on_hover_text(localize("options.output_dir.tooltip"));

                if ui.button("options.output_dir").clicked() {
                    let current_output_dir = {
                        let guard = &app.args.lock().unwrap().output;
                        match guard {
                            None => "".to_string(),
                            Some(it) => it.first().unwrap_or(&String::new()).to_string(),
                        }
                    };
                    if let Some(cache_dir_folder) =
                        rfd::FileDialog::new().set_directory(current_output_dir).pick_folder()
                        && let Some(cache_dir) = cache_dir_folder.to_str()
                    {
                        let guard = &mut app.args.lock().unwrap();
                        guard.output = Some(vec![cache_dir.to_string()]);
                        // app.set_cache_dir = cache_dir.to_string();
                    }
                }
            });

            // packages_filter: vec![],

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.no_deobf")))
                    .on_hover_text(localize("options.no_deobf.tooltip"));
                components::toggle_ui_compact(ui, &mut app.no_deobf);
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.progress")))
                    .on_hover_text(localize("options.progress.tooltip"));
                components::toggle_ui_compact(ui, &mut app.progress);
            });

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.strip_resources")))
                    .on_hover_text(localize("options.strip_resources.tooltip"));
                components::toggle_ui_compact(ui, &mut app.strip_resources);
            });
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(localize("options.deflate_compress_level")))
                    .on_hover_text(localize("options.deflate_compress_level.tooltip"));
                egui::ComboBox::from_id_salt("deflate_compress_level")
                    .selected_text(format!("{}", &app.deflate_compress_level))
                    .show_ui(ui, |ui| {
                        for i in 0..=12 {
                            ui.selectable_value(&mut app.deflate_compress_level, i, format!("{}", i));
                        }
                    });
            });
        });
}
