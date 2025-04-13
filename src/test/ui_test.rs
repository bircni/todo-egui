use egui::{Context, TextStyle, ThemePreference, accesskit::Role, vec2};
use egui_kittest::{Harness, kittest::Queryable};
use wgpu::InstanceDescriptor;

use crate::ui::{App, PLUS};

impl App {
    pub fn test(cc: &Context) -> Self {
        egui_extras::install_image_loaders(cc);
        cc.style_mut(|s| {
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

        cc.set_fonts(fonts);

        Self::default()
    }
}

pub fn app() -> Harness<'static> {
    let mut app = None;

    Harness::new(move |ctx| {
        let app_instance = app.get_or_insert_with(|| App::test(ctx));
        app_instance.show(ctx);
    })
}

async fn gpu_available() -> bool {
    wgpu::Instance::new(&InstanceDescriptor::default())
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .is_ok()
}

#[tokio::test]
pub async fn test_main_view() {
    if !gpu_available().await {
        return;
    }

    let themes = vec![ThemePreference::Dark, ThemePreference::Light];

    for theme in themes {
        let mut harness = app();
        harness.ctx.set_theme(theme);

        harness.run();
        harness.snapshot(&format!("{theme:?}_main_view"));

        harness
            .get_by_role_and_label(Role::Button, &format!("{PLUS} Add category"))
            .click();

        harness.run();
        harness.snapshot(&format!("{theme:?}_add_category"));

        harness
            .get_by_role_and_label(Role::Button, &format!("{PLUS} Add item"))
            .click();

        harness.run();
        harness.snapshot(&format!("{theme:?}_add_item"));

        harness.get_by_role_and_label(Role::Button, " ? ").click();

        harness.run();
        harness.snapshot(&format!("{theme:?}_about_dialog"));
    }
}
