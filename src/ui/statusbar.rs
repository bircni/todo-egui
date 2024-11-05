use egui::{special_emojis::GITHUB, vec2, Align, Align2, Button, Frame, Layout, Ui, Window};

use crate::data::{Category, List};
use crate::ui::PLUS;

const SAVE: &str = egui_phosphor::regular::FLOPPY_DISK;
const QUIT: &str = egui_phosphor::regular::SIGN_OUT;
const LOAD: &str = egui_phosphor::regular::FOLDER_OPEN;

pub struct StatusBar {
    show_about: bool,
}

impl StatusBar {
    pub const fn new() -> Self {
        Self { show_about: false }
    }

    pub fn show(&mut self, ui: &mut Ui, list: &mut List) -> anyhow::Result<()> {
        let mut ret = Ok(());
        self.about_window(ui);

        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                if ui.add(Button::new(format!("{SAVE} Save"))).clicked() {
                    ret = list.write();
                }
                if ui.add(Button::new(format!("{LOAD} Load"))).clicked() {
                    ret = list.load();
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
