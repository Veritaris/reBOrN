use crate::app::{App, DeobfMappingsType};
use crate::components;
use crate::components::file_select::SelectFileFilter;
use crate::localisation::localize;
use eframe::emath::Align;
use egui::Ui;
use std::collections::HashMap;

fn deobf_target_versions_selector(ui: &mut Ui, app: &mut App) {
    ui.visuals_mut().indent_has_left_vline = false;
    ui.indent(ui.id(), |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                let label = ui.label(egui::RichText::new(localize("Version")));
                let selector_text = app.last_used_version.clone().unwrap_or_default();
                egui::ComboBox::from_id_salt(label.id)
                    .selected_text(selector_text)
                    .show_ui(ui, |ui| {
                        let versions_json = app.versions_json.lock().unwrap();

                        for (version, channels) in &*versions_json {
                            let combobox = ui.add(egui::Button::selectable(
                                app.last_used_version.as_ref() == Some(version),
                                version,
                            ));
                            if combobox.clicked() && app.last_used_version.as_ref() != Some(version) {
                                app.last_used_version = Some(version.clone());
                                app.last_used_channel = match channels.keys().next() {
                                    None => None,
                                    Some(channel) => {
                                        app.last_used_mapping_version = match channels.get(channel) {
                                            None => None,
                                            Some(mapping_channels) => mapping_channels.first().cloned(),
                                        };
                                        Some(channel.clone())
                                    }
                                };
                            }
                        }
                    });
            });

            ui.vertical(|ui| {
                let label = ui.label(egui::RichText::new(localize("Channel")));
                let selector_text = app.last_used_channel.clone().unwrap_or_default();
                egui::ComboBox::from_id_salt(label.id)
                    .selected_text(selector_text)
                    .show_ui(ui, |ui| {
                        let versions_json = app.versions_json.lock().unwrap();
                        let version = &app.last_used_version.clone().unwrap_or_default();
                        let default_channels = &HashMap::<String, Vec<String>>::new();

                        let channels = versions_json.get(version).unwrap_or(default_channels);

                        for (channel, versions) in channels {
                            let combobox = ui.add(egui::Button::selectable(
                                app.last_used_channel.as_ref() == Some(channel),
                                channel,
                            ));
                            if combobox.clicked() && app.last_used_channel.as_ref() != Some(channel) {
                                app.last_used_channel = Some(channel.clone());
                                let version = versions.first().unwrap_or(&"".to_string()).clone();
                                app.last_used_mapping_version = Some(version)
                            }
                        }
                    });
            });

            ui.vertical(|ui| {
                let label = ui.label(egui::RichText::new(localize("MapVersion")));
                let selector_text = app.last_used_mapping_version.clone().unwrap_or_default();
                egui::ComboBox::from_id_salt(label.id)
                    .selected_text(selector_text)
                    .show_ui(ui, |ui| {
                        let versions_json = app.versions_json.lock().unwrap();
                        let default_channels = &HashMap::<String, Vec<String>>::new();
                        let default_map_versions = Vec::<String>::new();
                        let version = &app.last_used_version.clone().unwrap_or_default();
                        let channel = &app.last_used_channel.clone().unwrap_or_default();
                        let map_versions = versions_json
                            .get(version)
                            .unwrap_or(default_channels)
                            .get(channel)
                            .unwrap_or(&default_map_versions);

                        for map_ver in map_versions {
                            ui.selectable_value(&mut app.last_used_mapping_version, Some(map_ver.clone()), map_ver);
                        }
                    });
            });

            ui.vertical(|ui| {
                ui.with_layout(egui::Layout::bottom_up(Align::LEFT), |ui| {
                    if ui.button("Download now").clicked() {
                        app.download_mappings();
                    };
                });
            });

            ui.vertical(|ui| {
                ui.with_layout(egui::Layout::bottom_up(Align::LEFT), |ui| {
                    if ui.button("Open folder").clicked() {
                        utils::utils::open_explorer(utils::cache::get_appdata_dir().cache_dir());
                    };
                });
            });
        });
    });
}

enum FileSelectorAction {
    None,
    Select(Vec<std::path::PathBuf>),
    Clear(std::path::PathBuf, usize),
}

const CSV_SELECTION_FILTER: &[SelectFileFilter; 1] = &[SelectFileFilter {
    name: "csv",
    extensions: &["csv"],
}];
fn mapping_file_selector(ui: &mut egui::Ui, label: &str, file: &Option<std::path::PathBuf>) -> FileSelectorAction {
    let mut action = FileSelectorAction::None;
    ui.horizontal(|ui| {
        components::file_select::file_select(label, false, CSV_SELECTION_FILTER, ui, |files| {
            if !files.is_empty() {
                action = FileSelectorAction::Select(files.clone());
            }
        });
        if let Some(files) = file {
            let file_slice = std::slice::from_ref(files);
            ui.horizontal(|ui| {
                components::picked_files(ui, file_slice, None, |fpath, fid| {
                    action = FileSelectorAction::Clear(fpath.to_path_buf(), fid)
                });
            });
        }
    });

    action
}

fn deobf_custom_mappings_selector(ui: &mut egui::Ui, app: &mut App) {
    match mapping_file_selector(ui, "fields", &app.fields_file) {
        FileSelectorAction::None => (),
        FileSelectorAction::Select(files) => {
            if !files.is_empty() {
                app.fields_file = Some(files.first().unwrap().clone());
            }
        }
        FileSelectorAction::Clear(_, _) => app.fields_file = None,
    };

    match mapping_file_selector(ui, "methods", &app.methods_file) {
        FileSelectorAction::None => (),
        FileSelectorAction::Select(files) => {
            if !files.is_empty() {
                app.methods_file = Some(files.first().unwrap().clone());
            }
        }
        FileSelectorAction::Clear(_, _) => app.methods_file = None,
    };

    match mapping_file_selector(ui, "params", &app.params_file) {
        FileSelectorAction::None => (),
        FileSelectorAction::Select(files) => {
            if !files.is_empty() {
                app.params_file = Some(files.first().unwrap().clone());
            }
        }
        FileSelectorAction::Clear(_, _) => app.params_file = None,
    };
}

pub fn remap_target_select(ui: &mut Ui, app: &mut App) {
    egui::CollapsingHeader::new(localize("deobfuscation.remap_to"))
        .default_open(true)
        .show_unindented(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut app.deobf_target_select_state,
                    DeobfMappingsType::VersionsJSON,
                    "Versions JSON",
                );
                ui.selectable_value(
                    &mut app.deobf_target_select_state,
                    DeobfMappingsType::Custom,
                    "Custom",
                );
            });
            ui.separator();

            match app.deobf_target_select_state {
                DeobfMappingsType::VersionsJSON => deobf_target_versions_selector(ui, app),
                DeobfMappingsType::Custom => deobf_custom_mappings_selector(ui, app),
            }
        });
}
