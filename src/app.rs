use image::{ImageBuffer, Rgb};

use crate::{calibration_pattern::generate_calibration_pattern, camera::capture_camera_image};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    #[serde(skip)]
    camera_image: Option<ImageBuffer<Rgb<u8>, Vec<u8>>>,
    #[serde(skip)]
    texture: Option<egui::TextureHandle>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            camera_image: Default::default(),
            texture: Default::default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            camera_image,
            texture,
        } = self;

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Camera controls");

            if ui.button("Capture camera image").clicked() {
                *camera_image = Some(capture_camera_image());
                *texture = None;
            }
            if ui.button("Reset image").clicked() {
                *camera_image = None;
                *texture = None;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
                egui::warn_if_debug_build(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(camera_image) = camera_image {
                let texture: &egui::TextureHandle = texture.get_or_insert_with(|| {
                    // if let Some(state) =_frame.wgpu_render_state() {
                    //     state.renderer.write().register_native_texture(device, texture, texture_filter)
                    // }
                    let size = [camera_image.width() as _, camera_image.height() as _];
                    let pixels = camera_image.as_flat_samples();
                    ui.ctx().load_texture(
                        "camera-image",
                        egui::ColorImage::from_rgb(size, pixels.as_slice()),
                        Default::default(),
                    )
                });
                // Show the image:
                ui.image(texture, texture.size_vec2());
            } else {
                let (img_width, img_height) = if ui.available_size().is_finite() {
                    (ui.available_width() as _, ui.available_height() as _)
                } else {
                    (1280, 1280)
                };
                if let Some(t) = texture {
                    let [texture_width, texture_height] = t.size();
                    if img_width != texture_width as _ || img_height != texture_height as _ {
                        *texture = None;
                    }
                }
                let texture: &egui::TextureHandle = texture.get_or_insert_with(|| {
                    // if let Some(state) =_frame.wgpu_render_state() {
                    //     state.renderer.write().register_native_texture(device, texture, texture_filter)
                    // }
                    let image_buffer = generate_calibration_pattern(img_width, img_height);
                    let size = [image_buffer.width() as _, image_buffer.height() as _];
                    let pixels = image_buffer.as_flat_samples();
                    ui.ctx().load_texture(
                        "calibration-patter-image",
                        egui::ColorImage::from_rgb(size, pixels.as_slice()),
                        Default::default(),
                    )
                });
                // Show the image:
                ui.image(texture, texture.size_vec2());
            }
        });
    }
}
