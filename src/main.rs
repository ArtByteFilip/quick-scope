use eframe::egui;
use screenshots::Screen;
use std::time::Duration;

// Window settings
const WINDOW_SIZE: f32 = 500.0;
const WINDOW_REFRESH_RATE_MS: u64 = 10;
const WINDOW_INITIAL_X: f32 = 900.0 - WINDOW_SIZE;  // Initial X position
const WINDOW_INITIAL_Y: f32 = 900.0 - WINDOW_SIZE;  // Initial Y position

// Capture settings
const CAPTURE_SIZE: i32 = 200;
const CAPTURE_OFFSET: i32 = CAPTURE_SIZE / 2; // Half of capture size to center it

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
                let size = egui::vec2(WINDOW_SIZE, WINDOW_SIZE);
                let available_size = ui.available_size();
                let image_pos = (available_size - size) / 2.0;
                ui.allocate_space(image_pos);
                ui.image((texture.id(), size));
            }
        });

        // Request continuous repaints even when not focused
        ctx.request_repaint();

        // Capture screen every WINDOW_REFRESH_RATE_MS
        if self.last_capture.elapsed() > Duration::from_millis(WINDOW_REFRESH_RATE_MS) {
            if let Ok(screens) = Screen::all() {
                if let Some(screen) = screens.first() {
                    let center_x = screen.display_info.x + (screen.display_info.width as i32 / 2);
                    let center_y = screen.display_info.y + (screen.display_info.height as i32 / 2);
                    
                    let x = center_x - CAPTURE_OFFSET;
                    let y = center_y - CAPTURE_OFFSET;
                    
                    if let Ok(image) = screen.capture_area(x, y, CAPTURE_SIZE as u32, CAPTURE_SIZE as u32) {
                        let pixels = image.as_raw();
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            [CAPTURE_SIZE as usize, CAPTURE_SIZE as usize],
                            pixels,
                        );

                        self.screen_image = Some(ctx.load_texture(
                            "screen_capture",
                            color_image,
                            egui::TextureOptions::default(),
                        ));
                    }
                }
            }
            self.last_capture = std::time::Instant::now();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_SIZE, WINDOW_SIZE])
            .with_position(egui::Pos2::new(WINDOW_INITIAL_X, WINDOW_INITIAL_Y))
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
