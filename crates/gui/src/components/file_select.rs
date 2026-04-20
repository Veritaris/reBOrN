use crate::components;
use crate::localisation::localize;
use egui::Ui;

pub struct SelectFileFilter<'a> {
    pub name: &'a str,
    pub extensions: &'a [&'a str],
}

pub fn file_select<Consumer>(
    label_text: &str,
    select_many: bool,
    filters: &[SelectFileFilter],
    ui: &mut Ui,
    mut on_file_select: Consumer,
) where
    Consumer: FnMut(Vec<std::path::PathBuf>),
{
    if ui.button(label_text).clicked() {
        let mut files_dialog = rfd::FileDialog::new()
            .set_directory(".")
            .set_title(localize(label_text));

        for one_filter in filters {
            files_dialog = files_dialog.add_filter(one_filter.name, one_filter.extensions);
        }

        let files = if select_many {
            match files_dialog.pick_file() {
                None => None,
                Some(res) => Some(vec![res]),
            }
        } else {
            files_dialog.pick_files()
        };

        if let Some(files) = files {
            on_file_select(files);
        }
    }
}
