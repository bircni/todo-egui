use eframe::CreationContext;
use egui::{vec2, CentralPanel, Context, Id, PopupCloseBehavior, TextEdit, TextStyle};
use egui_notify::Toasts;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use statusbar::StatusBar;

use crate::data::{Item, List};

mod statusbar;

pub const PLUS: &str = egui_phosphor::regular::PLUS;
pub static APP_KEY: Lazy<String> = Lazy::new(|| format!("app-{}", env!("CARGO_PKG_NAME")));

#[derive(Deserialize, Serialize, Default)]
#[serde(default)]
pub struct App {
    list: List,
    new_item: Item,
    statusbar: StatusBar,
    #[serde(skip)]
    #[serde(default = "create_toasts")]
    toasts: Toasts,
}

pub const fn create_toasts() -> Toasts {
    Toasts::new().with_anchor(egui_notify::Anchor::BottomRight)
}

impl App {
    pub fn new(cc: &CreationContext) -> Self {
        cc.egui_ctx.style_mut(|s| {
            s.text_styles.insert(
                TextStyle::Name("subheading".into()),
                TextStyle::Monospace.resolve(s),
            );
            s.text_styles
                .insert(TextStyle::Body, TextStyle::Monospace.resolve(s));
            s.spacing.item_spacing = vec2(10.0, std::f32::consts::PI * 1.76643);
        });

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);

        cc.egui_ctx.set_fonts(fonts);

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, &APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

/// Main application loop (called every frame)
impl eframe::App for App {
    #[allow(clippy::significant_drop_in_scrutinee)]
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            self.statusbar.show(ui, &mut self.list).unwrap_or_else(|e| {
                self.toasts.error(e.to_string());
            });

            ui.vertical_centered(|ui| {
                ui.separator();
            });

            ui.horizontal(|ui| {
                for category in &mut self.list.categories {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                TextEdit::singleline(&mut category.name)
                                    .hint_text("Category")
                                    .desired_width(150.0),
                            );
                        });

                        for item in &mut category.items {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut item.todo, &item.name);

                                ui.button("📝")
                                    .on_hover_text("Edit notes")
                                    .clicked()
                                    .then(|| {
                                        ui.memory_mut(|m| {
                                            m.toggle_popup(egui::Id::new(&item.name));
                                        });
                                    });

                                // Notes popup
                                egui::popup::popup_below_widget(
                                    ui,
                                    Id::new(item.name.clone()),
                                    &ui.response(),
                                    PopupCloseBehavior::CloseOnClickOutside,
                                    |ui| {
                                        ui.set_min_width(200.0);
                                        ui.label("Notes:");
                                        ui.text_edit_multiline(&mut item.notes);
                                    },
                                );
                            });
                        }

                        let id = Id::new(format!("Add item to {}", category.name));
                        ui.button(format!("{PLUS} Add item"))
                            .on_hover_text("Add a new item")
                            .clicked()
                            .then(|| {
                                ui.memory_mut(|m| {
                                    m.toggle_popup(id);
                                });
                            });

                        egui::popup::popup_below_widget(
                            ui,
                            id,
                            &ui.response(),
                            PopupCloseBehavior::CloseOnClickOutside,
                            |ui| {
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
                                    ui.memory_mut(|m| {
                                        m.toggle_popup(id);
                                    });
                                }
                            },
                        );
                    });
                }
            });
        });
        self.toasts.show(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
