use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use egui::Context;
use egui::{special_emojis::GITHUB, vec2, Align, Align2, Button, Frame, Layout, Ui, Window};
use egui_file::{DialogType, FileDialog};
use serde::{Deserialize, Serialize};

use crate::data::{Category, List};
use crate::ui::PLUS;

const SAVE: &str = egui_phosphor::regular::FLOPPY_DISK;
const QUIT: &str = egui_phosphor::regular::SIGN_OUT;
const LOAD: &str = egui_phosphor::regular::FOLDER_OPEN;
const NEW: &str = egui_phosphor::regular::FILE_PLUS;

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct StatusBar {
    show_about: bool,
    file_loader: FileLoader,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
struct FileLoader {
    file_path: Option<PathBuf>,
    #[serde(skip)]
    file_dialog: Option<FileDialog>,
    new_file: bool,
}

impl FileLoader {
    pub fn show_file_dialog(&mut self, ctx: &Context, list: &mut List) -> anyhow::Result<()> {
        if let Some(dialog) = &mut self.file_dialog {
            if dialog.show(ctx).selected() {
                match dialog.dialog_type() {
                    DialogType::OpenFile => {
                        if let Some(file) = dialog.path() {
                            list.load(file)?;
                            self.file_path = Some(file.to_path_buf());
                            self.file_dialog = None;
                        }
                    }
                    DialogType::SaveFile => {
                        if let Some(file) = dialog.path() {
                            list.write(file, self.new_file)?;
                            self.file_path = Some(file.to_path_buf());
                            self.file_dialog = None;
                            self.new_file = false;
                        }
                    }
                    DialogType::SelectFolder => {}
                }
            }
        }
        Ok(())
    }
}

impl StatusBar {
    pub fn show(&mut self, ui: &mut Ui, list: &mut List) -> anyhow::Result<()> {
        let mut ret = self.file_loader.show_file_dialog(ui.ctx(), list);
        self.about_window(ui);

        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                if ui.add(Button::new(format!("{SAVE} Save"))).clicked() {
                    if let Some(path) = &self.file_loader.file_path {
                        ret = list.write(path, false);
                    } else {
                        let mut dialog = FileDialog::save_file(None).show_files_filter(Box::new({
                            let ext = Some(OsStr::new("json"));
                            move |path: &Path| -> bool { path.extension() == ext }
                        }));
                        dialog.open();
                        self.file_loader.file_dialog = Some(dialog);
                    }
                    ui.close_menu();
                }
                if ui.add(Button::new(format!("{NEW} New"))).clicked() {
                    let mut dialog = FileDialog::save_file(self.file_loader.file_path.clone())
                        .show_files_filter(Box::new({
                            let ext = Some(OsStr::new("json"));
                            move |path: &Path| -> bool { path.extension() == ext }
                        }));
                    dialog.open();
                    self.file_loader.file_dialog = Some(dialog);
                    self.file_loader.new_file = true;
                    ui.close_menu();
                }
                if ui.add(Button::new(format!("{LOAD} Load"))).clicked() {
                    let mut dialog = FileDialog::open_file(self.file_loader.file_path.clone())
                        .show_files_filter(Box::new({
                            let ext = Some(OsStr::new("json"));
                            move |path: &Path| -> bool { path.extension() == ext }
                        }));
                    dialog.open();
                    self.file_loader.file_dialog = Some(dialog);
                    ui.close_menu();
                }
                if ui.button(format!("{QUIT} Quit")).clicked() {
                    std::process::exit(0);
                }
            });

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add(Button::new(" ? ").rounding(40.0))
                    .clicked()
                    .then(|| self.show_about = true);

                ui.button(format!("{PLUS} Add category"))
                    .clicked()
                    .then(|| {
                        list.categories.push(Category::default());
                    });
            })
        });
        ret
    }

    fn about_window(&mut self, ui: &Ui) {
        Window::new("About")
            .resizable(false)
            .collapsible(false)
            .open(&mut self.show_about)
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .fixed_size(vec2(200.0, 150.0))
            .frame(Frame::window(ui.style()).fill(ui.style().visuals.widgets.open.weak_bg_fill))
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    // ui.add(
                    //     Image::new(include_image!("../../res/icon.png"))
                    //         .shrink_to_fit()
                    //         .rounding(10.0),
                    // );

                    ui.label(format!("{}: {}", "Version", env!("CARGO_PKG_VERSION")));
                    ui.hyperlink_to(
                        format!("{GITHUB} {}", "Github"),
                        "https://github.com/bircni/todo-egui",
                    );

                    ui.hyperlink_to("Built with egui", "https://docs.rs/egui/");
                });
            });
    }
}
