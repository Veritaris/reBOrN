use std::collections::HashMap;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

use egui::{CollapsingHeader, SelectableLabel, Style, Ui, ViewportCommand, Visuals};
use linked_hash_map::LinkedHashMap;

use utils::cache::{get_appdata_dir, get_cache_file_path, RebornCache};

use crate::components::toggle_ui_compact;
use crate::localisation::localize;

const ICON: &[u8] = include_bytes!("../../../resources/icon.iconset/icon-1024.png");
const APP_KEY: &str = "me.veritaris.reBOrN";

#[derive(Eq, PartialEq, Clone, Copy)]
enum AppState {
    Started,
    Ready,
    ValidatingCache,
    DownloadingMappings,
    Deobfuscating,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    cache_web_files: bool,

    autoupdate_versions: bool,
    last_used_version: Option<String>,
    last_used_channel: Option<String>,
    last_used_mapping_version: Option<String>,

    #[serde(skip)]
    extra_mappings_keys: Vec<String>,

    #[serde(skip)]
    extra_mappings_values: Vec<String>,

    #[serde(skip)]
    versions_json: Arc<Mutex<LinkedHashMap<String, HashMap<String, Vec<String>>>>>,

    #[serde(skip)]
    app_cache: Arc<Mutex<RebornCache>>,

    #[serde(skip)]
    state: Arc<Mutex<AppState>>,

    #[serde(skip)]
    versions_file_exists: bool,

    #[serde(skip)]
    mappings_cache_exists: bool,

    #[serde(skip)]
    tx: Sender<bool>,

    #[serde(skip)]
    rx: Receiver<bool>,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();


        Self {
            cache_web_files: true,
            autoupdate_versions: true,
            last_used_version: Some("1.7.10".to_string()),
            last_used_channel: Some("stable".to_string()),
            last_used_mapping_version: Some("12".to_string()),
            extra_mappings_keys: vec![],
            extra_mappings_values: vec![],
            versions_json: Default::default(),
            app_cache: Arc::new(Mutex::new(RebornCache::load())),
            state: Arc::new(Mutex::new(AppState::Started)),
            versions_file_exists: false,
            mappings_cache_exists: false,
            tx,
            rx,
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
            return eframe::get_value(storage, APP_KEY).unwrap_or_default();
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
                inner_size: Some(egui::vec2(1080.0, 720.0)),
                icon: Some(Arc::from(eframe::icon_data::from_png_bytes(ICON).unwrap())),
                ..Default::default()
            },
            centered: true,
            persist_window: true,
            ..Default::default()
        };
        eframe::run_native(
            APP_KEY,
            options,
            Box::new(|cc| {
                Ok(Box::new(App::new(cc)))
            }),
        )
    }

    fn ensure_versions_json(self: &mut App) {
        let cache: Arc<Mutex<RebornCache>> = Arc::clone(&self.app_cache);
        let state = Arc::clone(&self.state);
        let versions_json = Arc::clone(&self.versions_json);

        std::thread::spawn(move || {
            *state.lock().unwrap() = AppState::ValidatingCache;

            let cached_file_hash = cache.lock().unwrap().versions_json_hash.clone();
            match utils::cache::ensure_versions_json(Some(cached_file_hash.clone())) {
                Ok(file_hash) => {
                    if cached_file_hash != file_hash {
                        cache.lock().unwrap().versions_json_hash = file_hash;
                        cache.lock().unwrap().save_to_disk();
                        *cache.lock().unwrap() = RebornCache::read_from_file();
                    }
                    *versions_json.lock().unwrap() = utils::cache::read_versions_json();
                }
                Err(_) => {}
            };

            *state.lock().unwrap() = AppState::Ready;
        });
    }

    fn download_mappings(self: &mut App) {
        let state = Arc::clone(&self.state);

        let game_version = self.last_used_version.clone().unwrap();
        let mappings_channel = self.last_used_channel.clone().unwrap();
        let mappings_version = self.last_used_mapping_version.clone().unwrap();

        std::thread::spawn(move || {
            *state.lock().unwrap() = AppState::DownloadingMappings;
            utils::download::download_mappings(
                utils::cache::get_appdata_dir().cache_dir(),
                game_version.as_str(),
                mappings_channel.as_str(),
                mappings_version.as_str(),
            );
            *state.lock().unwrap() = AppState::Ready;
        });
    }

    fn mappings_download_button(self: &mut App, ui: &mut Ui) {
        if ui.button("Download now").clicked() {
            self.download_mappings();
        };
    }

    fn extra_mappings_editing(&mut self, ui: &mut Ui) {
        let len = (&self.extra_mappings_keys).len();
        for i in 0..len {
            match self.extra_mappings_keys.get_mut(i) {
                None => continue,
                Some(key) => {
                    ui.add_sized([120.0, 16.0], egui::TextEdit::singleline(key).code_editor());
                    ui.add_sized([240.0, 16.0], egui::TextEdit::singleline(self.extra_mappings_values.get_mut(i).unwrap()).code_editor());
                    if ui.add(egui::Button::new("remove")).clicked() {
                        self.extra_mappings_keys.remove(i);
                        self.extra_mappings_values.remove(i);
                    };
                    ui.end_row();
                }
            }
        };
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        window_frame(ctx, "reBOrN", |ui| {
            #[cfg(debug_assertions)]
            {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut *self.state.lock().unwrap(), AppState::Started, "Started");
                    ui.selectable_value(&mut *self.state.lock().unwrap(), AppState::ValidatingCache, "ValidatingCache");
                    ui.selectable_value(&mut *self.state.lock().unwrap(), AppState::Ready, "Ready");
                    ui.selectable_value(&mut *self.state.lock().unwrap(), AppState::DownloadingMappings, "DownloadingMappings");
                    ui.selectable_value(&mut *self.state.lock().unwrap(), AppState::Deobfuscating, "Deobfuscating");
                });
            }

            ui.horizontal(|ui| {
                ui.label(localize("options.autoupdate_versions"))
                    .on_hover_text(localize("options.autoupdate_versions.tooltip"));
                toggle_ui_compact(ui, &mut self.autoupdate_versions);
            });

            let app_state = *self.state.lock().unwrap();
            match app_state {
                AppState::Started => {
                    self.ensure_versions_json()
                }
                AppState::ValidatingCache => {
                    ui.horizontal_centered(|ui| {
                        ui.vertical_centered_justified(|ui| {
                            ui.label(localize("Downloading versions.json"));
                            ui.add(egui::Spinner::new().size(32.0));
                        });
                    });
                }
                AppState::DownloadingMappings => {}
                AppState::Deobfuscating => {}
                AppState::Ready => {
                    ui.label(localize("deobfuscation.input_files"));

                    ui.horizontal(|ui| {
                        ui.label(localize("deobfuscation.cache_web_files"))
                            .on_hover_text(localize("deobfuscation.cache_web_files.tooltip"));
                        toggle_ui_compact(ui, &mut self.cache_web_files);
                    });

                    CollapsingHeader::new(localize("deobfuscation.remap_from")).default_open(false).show_unindented(ui, |ui| {
                        ui.visuals_mut().indent_has_left_vline = false;
                        ui.indent(ui.id(), |ui| {
                            ui.vertical_centered_justified(|ui| {
                                ui.label("WIP, sorry")
                            });
                        });
                    });

                    CollapsingHeader::new(localize("deobfuscation.remap_to")).default_open(true).show_unindented(ui, |ui| {
                        ui.visuals_mut().indent_has_left_vline = false;
                        ui.indent(ui.id(), |ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    let label = ui.label(localize("Version"));
                                    let selector_text = self.last_used_version.clone().unwrap_or_default();
                                    egui::ComboBox::from_id_source(label.id)
                                        .selected_text(selector_text)
                                        .show_ui(ui, |ui| {
                                            let versions_json = self.versions_json.lock().unwrap();

                                            for (version, channels) in &*versions_json {
                                                let combobox = ui.add(SelectableLabel::new(self.last_used_version.as_ref() == Some(version), version));
                                                if combobox.clicked() && self.last_used_version.as_ref() != Some(&version) {
                                                    self.last_used_version = Some(version.clone());
                                                    self.last_used_channel = match channels.keys().next() {
                                                        None => None,
                                                        Some(channel) => {
                                                            self.last_used_mapping_version = match channels.get(channel).unwrap().first() {
                                                                None => None,
                                                                Some(map_version) => Some(map_version.clone())
                                                            };
                                                            Some(channel.clone())
                                                        }
                                                    };
                                                }
                                            }
                                        });
                                });

                                ui.vertical(|ui| {
                                    let label = ui.label(localize("Channel"));
                                    let selector_text = self.last_used_channel.clone().unwrap_or_default();
                                    egui::ComboBox::from_id_source(label.id)
                                        .selected_text(selector_text)
                                        .show_ui(ui, |ui| {
                                            let versions_json = self.versions_json.lock().unwrap();
                                            let version = &self.last_used_version.clone().unwrap_or_default();
                                            let default_channels = &HashMap::<String, Vec<String>>::new();

                                            let channels = &*versions_json
                                                .get(version).unwrap_or(default_channels);

                                            for (channel, versions) in channels {
                                                let combobox = ui.add(SelectableLabel::new(self.last_used_channel.as_ref() == Some(channel), channel));
                                                if combobox.clicked() && self.last_used_channel.as_ref() != Some(channel) {
                                                    self.last_used_channel = Some(channel.clone());
                                                    let version = versions.first().unwrap_or(&"".to_string()).clone();
                                                    self.last_used_mapping_version = Some(version)
                                                }
                                            }
                                        });
                                });

                                ui.vertical(|ui| {
                                    let label = ui.label(localize("MapVersion"));
                                    let selector_text = self.last_used_mapping_version.clone().unwrap_or_default();
                                    egui::ComboBox::from_id_source(label.id)
                                        .selected_text(selector_text)
                                        .show_ui(ui, |ui| {
                                            let versions_json = self.versions_json.lock().unwrap();
                                            let default_channels = &HashMap::<String, Vec<String>>::new();
                                            let default_map_versions = Vec::<String>::new();
                                            let version = &self.last_used_version.clone().unwrap_or_default();
                                            let channel = &self.last_used_channel.clone().unwrap_or_default();

                                            let map_versions = &*versions_json
                                                .get(version).unwrap_or(default_channels)
                                                .get(channel).unwrap_or(&default_map_versions);

                                            for map_ver in map_versions {
                                                ui.selectable_value(&mut self.last_used_mapping_version, Some(map_ver.clone()), map_ver);
                                            }
                                        });
                                });

                                ui.vertical(|ui| {
                                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                                        self.mappings_download_button(ui);
                                    });
                                });
                                
                                ui.vertical(|ui| {
                                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                                        // self.mappings_download_button(ui);
                                        if ui.button("Open folder").clicked() {
                                            utils::utils::open_explorer(get_appdata_dir().cache_dir());
                                        };
                                    });
                                });
                            });
                        });
                    });

                    ui.label(localize("deobfuscation.extra_mappings"));

                    egui::ScrollArea::horizontal()
                        .max_height(20.0)
                        .min_scrolled_height(20.0)
                        .show(ui, |ui| {
                            // egui_extras::TableBuilder::new(ui)
                            //     .min_scrolled_height(20.0)
                            //     .striped(true)
                            //     .header(16.0, |mut header| {
                            //         header.col(|ui| { ui.label("From"); });
                            //         header.col(|ui| { ui.label("To"); });
                            //     })
                            //     .body(|mut ui| {
                            //         self.extra_mappings_editing(&mut ui);
                            //     });
                            egui::Grid::new("extra_mappings")
                                .num_columns(3)
                                .spacing([1.0, 1.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label("From");
                                    ui.label("To");
                                    ui.end_row();
                                    self.extra_mappings_editing(ui);
                                });
                        });

                    // if ui.add(egui::Button::new("add mapping")).clicked() {
                    //     self.extra_mappings_keys.push("".to_string());
                    //     self.extra_mappings_values.push("".to_string());
                    // }

                    if self.extra_mappings_keys.is_empty()
                        || (self.extra_mappings_keys.last().unwrap_or(&"".to_string()) != &"".to_string() && self.extra_mappings_values.last().unwrap_or(&"".to_string()) != &"".to_string()) {
                        self.extra_mappings_keys.push("".to_string());
                        self.extra_mappings_values.push("".to_string());
                    }
                }
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, APP_KEY, self);
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
        let mut content_ui = ui.child_ui(content_rect, *ui.layout(), None);
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