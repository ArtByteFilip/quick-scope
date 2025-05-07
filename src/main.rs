use eframe::egui;
use screenshots::Screen;
use std::time::Duration;

struct ScreenCaptureApp {
    screen_image: Option<egui::TextureHandle>,
    last_capture: std::time::Instant,
}

impl Default for ScreenCaptureApp {
    fn default() -> Self {
        Self {
            screen_image: None,
            last_capture: std::time::Instant::now(),
        }
    }
}

impl eframe::App for ScreenCaptureApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.screen_image {
                let size = egui::vec2(500.0, 500.0);
                let available_size = ui.available_size();
                let image_pos = (available_size - size) / 2.0;
                ui.allocate_space(image_pos);
                ui.image((texture.id(), size));
            }
        });

        // Request continuous repaints even when not focused
        ctx.request_repaint();

        // Capture screen every 10ms
        if self.last_capture.elapsed() > Duration::from_millis(10) {
            if let Ok(screens) = Screen::all() {
                if let Some(screen) = screens.first() {
                    let center_x = screen.display_info.x + (screen.display_info.width as i32 / 2);
                    let center_y = screen.display_info.y + (screen.display_info.height as i32 / 2);
                    
                    let x = center_x - 100; // 200/2 = 100
                    let y = center_y - 100; // 200/2 = 100
                    
                    let image = screen.capture_area(x, y, 200, 200).unwrap();
                    let pixels = image.as_raw();

                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        [200, 200],
                        pixels,
                    );

                    self.screen_image = Some(ctx.load_texture(
                        "screen_capture",
                        color_image,
                        egui::TextureOptions::default(),
                    ));
                }
            }
            self.last_capture = std::time::Instant::now();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 500.0])
            .with_always_on_top()
            .with_transparent(true)
            .with_decorations(false)
            .with_window_level(egui::WindowLevel::AlwaysOnTop),
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        vsync: false,
        ..Default::default()
    };
    
    eframe::run_native(
        "Quick Scope",
        options,
        Box::new(|_cc| Box::new(ScreenCaptureApp::default())),
    )
}
