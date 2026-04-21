use crate::app::App;
use crate::components;
use crate::localisation::localize;
use crate::utils::is_empty_or_none;
use egui::Ui;
use mc_deobf::args::RebornCliArgs;
use mc_deobf::mappings::DeobfMappingsType;
use std::time::Duration;

const JAR_SELECT_FILTERS: [components::file_select::SelectFileFilter; 2] = [
    components::file_select::SelectFileFilter {
        name: "jar",
        extensions: &["jar", "zip"],
    },
    components::file_select::SelectFileFilter {
        name: "class",
        extensions: &["class"],
    },
];

pub fn app_ready(ui: &mut Ui, app: &mut App) {
    ui.label(egui::RichText::new(localize("deobfuscation.input_files")));

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(localize("deobfuscation.cache_web_files")))
            .on_hover_text(localize("deobfuscation.cache_web_files.tooltip"));
        components::toggle_ui_compact(ui, &mut app.cache_web_files);
    });

    egui::CollapsingHeader::new(localize("deobfuscation.remap_from"))
        .default_open(false)
        .show_unindented(ui, |ui| {
            ui.visuals_mut().indent_has_left_vline = false;
            ui.indent(ui.id(), |ui| {
                ui.vertical_centered_justified(|ui| ui.label("WIP, sorry"));
            });
        });

    components::remap_target_select(ui, app);

    components::remapper_args_block(ui, app);

    components::extra_mappings_editor(ui, app);

    if app.extra_mappings_keys.is_empty()
        || (app.extra_mappings_keys.last().unwrap_or(&"".to_string()) != &"".to_string()
            && app.extra_mappings_values.last().unwrap_or(&"".to_string()) != &"".to_string())
    {
        app.extra_mappings_keys.push("".to_string());
        app.extra_mappings_values.push("".to_string());
    }

    components::file_select::file_select("deobfuscation.select_file", false, &JAR_SELECT_FILTERS, ui, |files| {
        app.picked_files = Some(files)
    });

    if let Some(files) = &mut app.picked_files {
        ui.horizontal(|ui| {
            let mut index_to_remove: Vec<usize> = vec![];
            for i in 0..files.len() {
                components::picked_files(ui, files.as_slice(), Some(i), |_, fid| {
                    index_to_remove.push(fid);
                });
            }
            index_to_remove.sort_by(|a, b| b.cmp(a));
            for fid in index_to_remove {
                files.remove(fid);
            }
        });
    }

    if ui
        .add_enabled(
            !is_empty_or_none(&app.picked_files),
            egui::Button::new("deobfuscation.run"),
        )
        .on_disabled_hover_text("No files selected")
        .clicked()
        && let Some(files) = &app.picked_files
    {
        println!("Deobfuscating {:?}", files);
        let input: Vec<String> = files
            .iter()
            .map(|f| f.to_str().unwrap_or(""))
            .map(String::from)
            .collect::<Vec<_>>();

        let mut custom_mappings_as_extra_mappings: Vec<String> = vec![];
        if let Some(fields_mappings_file) = &app.fields_file {
            custom_mappings_as_extra_mappings
                .push(format!("fields:file://{}", fields_mappings_file.to_str().unwrap_or("")));
        }
        if let Some(methods_mappings_file) = &app.methods_file {
            custom_mappings_as_extra_mappings.push(format!(
                "methods:file://{}",
                methods_mappings_file.to_str().unwrap_or("")
            ));
        }
        if let Some(params_mappings_file) = &app.params_file {
            custom_mappings_as_extra_mappings
                .push(format!("params:file://{}", params_mappings_file.to_str().unwrap_or("")));
        }

        let output = {
            match app.args.lock() {
                Ok(it) => it.output.clone(),
                Err(err) => {
                    eprintln!("Unable to lock app.args mutex {}", err);
                    None
                }
            }
        };

        let deobf_suffix = if app.deobf_suffix_value.is_empty() {
            None
        } else {
            Some(app.deobf_suffix_value.clone())
        };

        let deobf_args = RebornCliArgs {
            input: input.clone(),
            verbose: app.verbose,
            debug: app.debug,
            print_class: app.print_class,
            print_cpool: app.print_cpool,
            print_code: app.print_code,
            mod_loader: app.mod_loader,
            cache_web_files: app.cache_web_files,
            show_versions: app.show_versions,
            cache_dir: app.cache_dir,
            // set_cache_dir: app.set_cache_dir,
            clean_cache_dir: app.clean_cache_dir,
            // packages_filter: app.packages_filter,
            no_deobf: app.no_deobf,
            progress: app.progress,
            strip_resources: app.strip_resources,
            deflate_compress_level: app.deflate_compress_level,
            compress_resources: app.compress_resources,
            mappings_type: app.deobf_target_select_state,
            extra_mappings: custom_mappings_as_extra_mappings,
            deobf_suffix,
            output,
            ..Default::default()
        };
        if !deobf_args.input.is_empty() {
            let mut input_files: Vec<String> = vec![];
            if deobf_args.mappings_type == DeobfMappingsType::VersionsJSON
                && !utils::cache::mappings_exists(
                    deobf_args.game_version.as_str(),
                    deobf_args.mappings_channel.as_str(),
                    deobf_args.mappings_version.as_str(),
                )
            {
                mc_deobf::remapper::prepare_mappings(&deobf_args);
            }
            mc_deobf::remapper::gather_input_files(&mut input_files, &deobf_args.input);
            mc_deobf::remapper::remap_files(&deobf_args, &input_files);
            app.toasts
                .lock()
                .map(|mut toasts| {
                    toasts
                        .success("deobfuscation.finished.success")
                        .duration(Duration::from_secs(5));
                })
                .expect("Unable to show toasts");
        }
    }
}
