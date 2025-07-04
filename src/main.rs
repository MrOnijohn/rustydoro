use eframe::egui;
use std::time::{Duration, Instant};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([150.0, 1177.0])
            .with_min_inner_size([50.0, 150.0])
            .with_resizable(true)
            .with_title("Rustydoro"),
        ..Default::default()
    };

    eframe::run_native(
        "Rustydoro",
        options,
        Box::new(|_cc| Ok(Box::new(PomodoroApp::default()))),
    )
}

#[derive(Debug, Clone, Copy)]
enum TimerState {
    Idle,
    Running,
    Paused,
}

struct PomodoroApp {
    state: TimerState,
    start_time: Option<Instant>,
    elapsed_when_paused: Duration,
    total_duration: Duration,
}

impl Default for PomodoroApp {
    fn default() -> Self {
        Self {
            state: TimerState::Idle,
            start_time: None,
            elapsed_when_paused: Duration::ZERO,
            total_duration: Duration::from_secs(30 * 60), // 30 minutes
        }
    }
}

impl PomodoroApp {
    fn get_elapsed_time(&self) -> Duration {
        match self.state {
            TimerState::Idle => Duration::ZERO,
            TimerState::Running => {
                if let Some(start) = self.start_time {
                    self.elapsed_when_paused + start.elapsed()
                } else {
                    self.elapsed_when_paused
                }
            }
            TimerState::Paused => self.elapsed_when_paused,
        }
    }

    fn get_progress(&self) -> f32 {
        let elapsed = self.get_elapsed_time();
        if elapsed >= self.total_duration {
            1.0
        } else {
            elapsed.as_secs_f32() / self.total_duration.as_secs_f32()
        }
    }

    fn start_timer(&mut self) {
        match self.state {
            TimerState::Idle => {
                self.state = TimerState::Running;
                self.start_time = Some(Instant::now());
                self.elapsed_when_paused = Duration::ZERO;
            }
            TimerState::Paused => {
                self.state = TimerState::Running;
                self.start_time = Some(Instant::now());
            }
            TimerState::Running => {} // Already running
        }
    }

    fn pause_timer(&mut self) {
        if matches!(self.state, TimerState::Running) {
            self.state = TimerState::Paused;
            if let Some(start) = self.start_time {
                self.elapsed_when_paused += start.elapsed();
            }
            self.start_time = None;
        }
    }

    fn reset_timer(&mut self) {
        self.state = TimerState::Idle;
        self.start_time = None;
        self.elapsed_when_paused = Duration::ZERO;
    }

    fn sigmoid_ease(t: f32) -> f32 {
        // Subtle sigmoid-like easing: slower at start, linear in middle, faster at end
        // Fixed version that maps 0->0 and 1->1 properly
        let k = 2.0; // Controls how subtle the curve is (reduced from 3.0)
        let sigmoid = 1.0 / (1.0 + (-k * (t - 0.5)).exp());
        // Normalize to 0-1 range
        let sigmoid_0 = 1.0 / (1.0 + (-k * (-0.5)).exp());
        let sigmoid_1 = 1.0 / (1.0 + (-k * (0.5)).exp());
        (sigmoid - sigmoid_0) / (sigmoid_1 - sigmoid_0)
    }

    // Just replace the get_color method in your existing code with this:

    fn get_color(&self) -> egui::Color32 {
        // Reset to turquoise when idle, paused, or timer complete
        if matches!(self.state, TimerState::Idle | TimerState::Paused) {
            return egui::Color32::from_rgb(64, 224, 208); // Turquoise
        }

        let progress = self.get_progress();

        // Timer complete - back to turquoise
        if progress >= 1.0 {
            return egui::Color32::from_rgb(64, 224, 208); // Turquoise
        }

        // Apply subtle sigmoid easing
        let eased_progress = Self::sigmoid_ease(progress);

        // Color transitions when running:
        // 0-0.5 (0-15 min): Forest green to vivid yellow
        // 0.5-0.833 (15-25 min): Vivid yellow to pure red
        // 0.833-1.0 (25-30 min): Pure red to black

        if eased_progress <= 0.5 {
            // Forest Green to Vivid Yellow (first 15 minutes)
            let t = eased_progress * 2.0; // 0.0 to 1.0
                                          // Start with forest green (34, 139, 34), end with vivid yellow (255, 255, 0)
            let r = (34.0 * (1.0 - t) + 255.0 * t) as u8;
            let g = (139.0 * (1.0 - t) + 255.0 * t) as u8;
            let b = (34.0 * (1.0 - t) + 0.0 * t) as u8;
            egui::Color32::from_rgb(r, g, b)
        } else if eased_progress <= 0.833 {
            // Vivid Yellow to Pure Red (next 10 minutes)
            let t = (eased_progress - 0.5) / 0.333; // 0.0 to 1.0
                                                    // Start with vivid yellow (255, 255, 0), end with pure red (255, 0, 0)
            let r = 255;
            let g = (255.0 * (1.0 - t) + 0.0 * t) as u8;
            let b = 0;
            egui::Color32::from_rgb(r, g, b)
        } else {
            // Pure Red to Black (last 5 minutes)
            let t = (eased_progress - 0.833) / 0.167; // 0.0 to 1.0
                                                      // Start with pure red (255, 0, 0), end with black (0, 0, 0)
            let r = (255.0 * (1.0 - t) + 0.0 * t) as u8;
            let g = 0;
            let b = 0;
            egui::Color32::from_rgb(r, g, b)
        }
    }

    fn format_time(&self, duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let minutes = total_secs / 60;
        let seconds = total_secs % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
}

impl eframe::App for PomodoroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle global shortcuts
        ctx.input(|i| {
            if i.modifiers.ctrl {
                if i.key_pressed(egui::Key::Q) {
                    std::process::exit(0);
                } else if i.modifiers.alt {
                    if i.key_pressed(egui::Key::S) {
                        self.start_timer();
                    } else if i.key_pressed(egui::Key::P) {
                        self.pause_timer();
                    }
                }
            }
        });

        // Check if timer is complete
        if matches!(self.state, TimerState::Running)
            && self.get_elapsed_time() >= self.total_duration
        {
            self.reset_timer();
        }

        // Only request repaints when the timer is actually running
        if matches!(self.state, TimerState::Running) {
            ctx.request_repaint_after(Duration::from_millis(100));
        }

        // Main panel that fills the entire window
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                // Override UI spacing for very narrow windows
                ui.spacing_mut().item_spacing = egui::vec2(2.0, 2.0);
                ui.spacing_mut().button_padding = egui::vec2(2.0, 2.0);

                // Set background color
                let bg_color = self.get_color();
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    egui::Rounding::ZERO,
                    bg_color,
                );

                // Handle mouse clicks
                let response = ui.allocate_response(ui.available_size(), egui::Sense::click());
                if response.clicked() {
                    // Left click: start/pause
                    match self.state {
                        TimerState::Idle | TimerState::Paused => self.start_timer(),
                        TimerState::Running => self.pause_timer(),
                    }
                } else if response.secondary_clicked() {
                    // Right click: reset
                    self.reset_timer();
                }

                // Timer display at bottom - adjust for narrow windows
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add_space(10.0); // Reduced spacing for narrow windows

                    let elapsed = self.get_elapsed_time();
                    let remaining = if elapsed >= self.total_duration {
                        Duration::ZERO
                    } else {
                        self.total_duration - elapsed
                    };

                    let time_text = self.format_time(remaining);

                    // Simple approach: use RichText with background
                    ui.visuals_mut().override_text_color = Some(egui::Color32::from_gray(140));

                    // Adjust text size and background based on available width
                    let available_width = ui.available_width();
                    let (text_size, bg_width, bg_height) = if available_width < 80.0 {
                        // Very narrow - use compact display
                        (10.0, available_width - 4.0, 20.0)
                    } else if available_width < 120.0 {
                        // Narrow - smaller text
                        (12.0, available_width - 10.0, 25.0)
                    } else {
                        // Normal width
                        (18.0, 80.0, 30.0)
                    };

                    let response =
                        ui.allocate_response(egui::vec2(bg_width, bg_height), egui::Sense::hover());

                    // Draw black background with smaller padding for narrow windows
                    let padding = if available_width < 80.0 { 2.0 } else { 8.0 };
                    ui.painter().rect_filled(
                        response.rect.expand(padding),
                        egui::Rounding::same(4.0),
                        egui::Color32::BLACK,
                    );

                    // Draw text
                    ui.put(
                        response.rect,
                        egui::Label::new(
                            egui::RichText::new(time_text)
                                .size(text_size)
                                .color(egui::Color32::from_gray(140)),
                        ),
                    );
                });
            });
    }
}
