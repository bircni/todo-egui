use eframe::CreationContext;
use egui::{CentralPanel, Context, Id, PopupCloseBehavior, TextEdit, TextStyle, vec2};
use egui_notify::Toasts;
use serde::{Deserialize, Serialize};
use statusbar::FileLoader;
use std::sync::LazyLock;

use crate::data::{Item, List};

mod statusbar;

pub const PLUS: &str = "➕";
pub static APP_KEY: LazyLock<String> = LazyLock::new(|| format!("app-{}", env!("CARGO_PKG_NAME")));

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct App {
    list: List,
    new_item: Item,
    #[serde(skip)]
    #[serde(default = "create_toasts")]
    toasts: Toasts,
    show_about: bool,
    pub file_loader: FileLoader,
}

pub const fn create_toasts() -> Toasts {
    Toasts::new().with_anchor(egui_notify::Anchor::BottomRight)
}

impl App {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.style_mut(|s| {
            s.text_styles.insert(
                TextStyle::Name("subheading".into()),
                TextStyle::Monospace.resolve(s),
            );
            s.text_styles
                .insert(TextStyle::Body, TextStyle::Monospace.resolve(s));
            s.spacing.item_spacing = vec2(10.0, std::f32::consts::PI * 1.76643);
        });

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            log::debug!("Loading app state from storage");
            return eframe::get_value(storage, &APP_KEY).unwrap_or_default();
        }

        Self::default()
    }

    pub fn show(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            self.show_statusbar(ui).unwrap_or_else(|e| {
                self.toasts.error(e.to_string());
            });

            ui.vertical_centered(|ui| {
                ui.separator();
            });

            let mut delete_category = None;
            ui.horizontal(|ui| {
                for category in &mut self.list.categories {
                    let mut delete_item = None;
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                TextEdit::singleline(&mut category.name)
                                    .hint_text("Category")
                                    .desired_width(150.0),
                            );
                            ui.button("❌")
                                .on_hover_text("Delete category")
                                .clicked()
                                .then(|| delete_category = Some(category.id));
                        });
                        for item in &mut category.items {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut item.todo, &item.name);

                                ui.button("📝")
                                    .on_hover_text("Edit notes")
                                    .clicked()
                                    .then(|| {
                                        egui::Popup::toggle_id(ctx, egui::Id::new(item.id));
                                    });
                                ui.button("❌")
                                    .on_hover_text("Delete item")
                                    .clicked()
                                    .then(|| delete_item = Some(item.id));

                                // Notes popup
                                egui::Popup::from_toggle_button_response(&ui.response())
                                    .id(egui::Id::new(item.id))
                                    .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                                    .show(|ui| {
                                        ui.set_min_width(200.0);
                                        ui.label("Notes:");
                                        ui.text_edit_multiline(&mut item.notes);
                                    });
                            });
                        }

                        if let Some(delete) = delete_item {
                            category.items.retain(|i| i.id != delete);
                        }

                        let id = Id::new(format!("Add item to {}", category.name));
                        ui.button(format!("{PLUS} Add item"))
                            .on_hover_text("Add a new item")
                            .clicked()
                            .then(|| {
                                egui::Popup::toggle_id(ctx, id);
                            });

                        egui::Popup::from_toggle_button_response(&ui.response())
                            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
                            .id(id)
                            .show(|ui| {
                                ui.set_min_width(200.0);
                                ui.label("Add item:");
                                ui.add(
                                    TextEdit::singleline(&mut self.new_item.name).hint_text("Name"),
                                );
                                ui.add(
                                    TextEdit::multiline(&mut self.new_item.notes)
                                        .hint_text("Notes"),
                                );
                                if ui.button(format!("{PLUS} Add")).clicked() {
                                    category.items.push(self.new_item.clone());
                                    self.new_item = Item::default();
                                    egui::Popup::toggle_id(ctx, id);
                                }
                            });
                    });
                }
                if let Some(delete) = delete_category {
                    self.list.categories.retain(|c| c.id != delete);
                }
            });
        });
        self.toasts.show(ctx);
    }
}

/// Main application loop (called every frame)
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.show(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &APP_KEY, self);
    }
}
