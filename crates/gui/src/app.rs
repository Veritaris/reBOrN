use crate::components;
use crate::containers::app_stages;
use crate::localisation::localize;
use eframe::emath::Align;
use eframe::epaint::Rgba;
use egui_notify::{Anchor, Toasts};
use linked_hash_map::LinkedHashMap;
use mc_deobf::args::RebornCliArgs;
pub(crate) use mc_deobf::mappings::{DeobfMappingsType, ModLoader};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;
use utils::cache::RebornCache;

const ICON: &[u8] = include_bytes!("../../../resources/icon.iconset/icon-1024.png");
const APP_KEY: &str = "me.veritaris.reBOrN";

#[derive(Eq, PartialEq, Clone, Copy)]
pub(crate) enum AppState {
    Started,
    Ready,
    ValidatingCache,
    DownloadingMappings,
    Deobfuscating,
}

type VersionsJSON = LinkedHashMap<String, HashMap<String, Vec<String>>>;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    autoupdate_versions: bool,
    pub(crate) last_used_version: Option<String>,
    pub(crate) last_used_channel: Option<String>,
    pub(crate) last_used_mapping_version: Option<String>,
    theme: egui::Theme,

    font_color: egui::Color32,

    // remap args
    pub(crate) verbose: u8,
    pub(crate) debug: bool,
    pub(crate) print_class: bool,
    pub(crate) print_cpool: bool,
    pub(crate) print_code: bool,
    pub(crate) mod_loader: ModLoader,
    pub(crate) cache_web_files: bool,
    pub(crate) show_versions: bool,
    pub(crate) cache_dir: bool,
    pub(crate) set_cache_dir: String,
    pub(crate) clean_cache_dir: bool,
    packages_filter: Vec<String>,
    // input: Vec,
    // output: Option,
    pub(crate) no_deobf: bool,
    pub(crate) progress: bool,
    pub(crate) strip_resources: bool,
    pub(crate) deflate_compress_level: i64,
    pub(crate) compress_resources: bool,

    #[serde(skip)]
    pub args: Arc<Mutex<RebornCliArgs>>,

    #[serde(skip)]
    pub(crate) extra_mappings_keys: Vec<String>,

    #[serde(skip)]
    pub(crate) extra_mappings_values: Vec<String>,

    #[serde(skip)]
    pub(crate) versions_json: Arc<Mutex<VersionsJSON>>,

    #[serde(skip)]
    dropped_files: Vec<egui::DroppedFile>,

    #[serde(skip)]
    pub(crate) picked_files: Option<Vec<std::path::PathBuf>>,

    #[serde(skip)]
    files_to_deobfuscate: Arc<Vec<String>>,

    #[serde(skip)]
    app_cache: Arc<Mutex<RebornCache>>,

    #[serde(skip)]
    pub(crate) state: Arc<Mutex<AppState>>,

    #[serde(skip)]
    pub(crate) deobf_target_select_state: DeobfMappingsType,

    #[serde(skip)]
    pub(crate) fields_file: Option<std::path::PathBuf>,

    #[serde(skip)]
    pub(crate) methods_file: Option<std::path::PathBuf>,

    #[serde(skip)]
    pub(crate) params_file: Option<std::path::PathBuf>,

    #[serde(skip)]
    versions_file_exists: bool,

    #[serde(skip)]
    mappings_cache_exists: bool,

    #[serde(skip)]
    pub(crate) toasts: Arc<Mutex<Toasts>>,

    #[serde(skip)]
    tx: Sender<bool>,

    #[serde(skip)]
    rx: Receiver<bool>,
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx): (Sender<bool>, Receiver<bool>) = mpsc::channel();

        Self {
            theme: egui::Theme::Dark,
            font_color: egui::Color32::WHITE,
            autoupdate_versions: true,
            last_used_version: Some("1.7.10".to_string()),
            last_used_channel: Some("stable".to_string()),
            last_used_mapping_version: Some("12".to_string()),
            extra_mappings_keys: vec![],
            extra_mappings_values: vec![],
            versions_json: Default::default(),
            dropped_files: vec![],
            picked_files: None,
            files_to_deobfuscate: Arc::new(Vec::<String>::default()),
            app_cache: Arc::new(Mutex::new(RebornCache::load())),
            state: Arc::new(Mutex::new(AppState::Started)),
            deobf_target_select_state: DeobfMappingsType::VersionsJSON,
            fields_file: None,
            methods_file: None,
            params_file: None,
            versions_file_exists: false,
            mappings_cache_exists: false,
            // remapping args
            args: Arc::new(Mutex::new(Default::default())),
            verbose: 0,
            debug: false,
            print_class: false,
            print_cpool: false,
            print_code: false,
            mod_loader: ModLoader::Forge,
            cache_web_files: false,
            show_versions: false,
            cache_dir: false,
            set_cache_dir: "".to_string(),
            clean_cache_dir: false,
            packages_filter: vec![],
            no_deobf: false,
            progress: false,
            strip_resources: false,
            deflate_compress_level: 0,
            compress_resources: false,
            toasts: Arc::new(Mutex::new(Toasts::default().with_anchor(Anchor::BottomRight))),
            tx,
            rx,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let style = egui::Style {
            visuals: egui::Visuals::dark(),
            ..egui::Style::default()
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
                inner_size: Some(egui::vec2(640.0, 480.0)),
                icon: Some(Arc::from(eframe::icon_data::from_png_bytes(ICON).unwrap())),
                maximized: Some(false),
                ..Default::default()
            },
            centered: true,
            persist_window: true,
            ..Default::default()
        };
        eframe::run_native(APP_KEY, options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
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
                Err(_) => {
                    todo!("show error to user")
                }
            };

            *state.lock().unwrap() = AppState::Ready;
        });
    }

    pub(crate) fn download_mappings(self: &mut App) {
        let state = Arc::clone(&self.state);
        let toasts = Arc::clone(&self.toasts);

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
                Some(|| {
                    toasts
                        .lock()
                        .map(|mut toasts| {
                            toasts
                                .success("downloads.mappings.success")
                                .duration(Duration::from_secs(10));
                        })
                        .expect("Unable to download mapping list");
                }),
            );
            *state.lock().unwrap() = AppState::Ready;
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        window_frame(ctx, "reBOrN", |ui| {
            #[cfg(debug_assertions)]
            {
                app_stages::debug_stages::debug_stages(ui, self);
            }
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(localize("options.autoupdate_versions")))
                        .on_hover_text(localize("options.autoupdate_versions.tooltip"));
                    components::toggle_ui_compact(ui, &mut self.autoupdate_versions);
                });

                let app_state = *self.state.lock().unwrap();
                match app_state {
                    AppState::Started => self.ensure_versions_json(),
                    AppState::ValidatingCache => app_stages::validating_cache::app_validating_cache(ui),
                    AppState::DownloadingMappings => {}
                    AppState::Deobfuscating => {}
                    AppState::Ready => app_stages::ready::app_ready(ui, ctx, self),
                }
                self.toasts
                    .lock()
                    .map(|mut toasts| {
                        toasts.show(ctx);
                    })
                    .expect("Unable to show toasts");
            });
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, APP_KEY, self);
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        Rgba::TRANSPARENT.to_array()
    }
}

fn window_frame(ctx: &egui::Context, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    let panel_frame = egui::Frame {
        fill: ctx.style().visuals.window_fill(),
        corner_radius: 10.0.into(),
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
        ui.ctx();
        title_bar_ui(ui, title_bar_rect, title);

        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);

        let mut content_ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect).layout(*ui.layout()));
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
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
    }

    if title_bar_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
    ui.scope_builder(egui::UiBuilder::new().max_rect(title_bar_rect), |ui| {
        ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
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
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(false));
        }
    } else {
        let maximized_response = ui
            .add(egui::Button::new(egui::RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(true));
        }
    }

    let minimized_response = ui
        .add(egui::Button::new(egui::RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }

    {
        let theme = ui.ctx().theme();

        #[allow(clippy::collapsible_else_if)]
        if theme == egui::Theme::Dark {
            if ui
                .add(egui::Button::new("☀").frame(false))
                .on_hover_text("Switch to light mode")
                .clicked()
            {
                ui.ctx().set_theme(egui::Theme::Light);
            }
        } else {
            if ui
                .add(egui::Button::new("🌙").frame(false))
                .on_hover_text("Switch to dark mode")
                .clicked()
            {
                ui.ctx().set_theme(egui::Theme::Dark);
            }
        }
    }
}
