use eframe::egui::{self, Color32, RichText};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Work,
    ShortBreak,
    LongBreak,
}

impl Mode {
    fn duration(self) -> Duration {
        match self {
            Mode::Work => Duration::from_secs(25 * 60),
            Mode::ShortBreak => Duration::from_secs(5 * 60),
            Mode::LongBreak => Duration::from_secs(15 * 60),
        }
    }

    fn label(self) -> &'static str {
        match self {
            Mode::Work => "Work",
            Mode::ShortBreak => "Short Break",
            Mode::LongBreak => "Long Break",
        }
    }

    fn color(self) -> Color32 {
        match self {
            Mode::Work => Color32::from_rgb(186, 73, 73),
            Mode::ShortBreak => Color32::from_rgb(56, 133, 138),
            Mode::LongBreak => Color32::from_rgb(57, 112, 151),
        }
    }
}

pub struct PomodoroApp {
    mode: Mode,
    remaining: Duration,
    running: bool,
    last_tick: Option<Instant>,
}

impl PomodoroApp {
    pub fn new() -> Self {
        Self {
            mode: Mode::Work,
            remaining: Mode::Work.duration(),
            running: false,
            last_tick: None,
        }
    }

    fn switch_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.remaining = mode.duration();
        self.running = false;
        self.last_tick = None;
    }

    fn toggle(&mut self) {
        self.running = !self.running;
        self.last_tick = if self.running { Some(Instant::now()) } else { None };
    }

    fn reset(&mut self) {
        self.remaining = self.mode.duration();
        self.running = false;
        self.last_tick = None;
    }

    fn tick(&mut self) {
        if !self.running {
            return;
        }
        let Some(last) = self.last_tick else { return };
        let elapsed = last.elapsed();
        self.remaining = self.remaining.saturating_sub(elapsed);
        if self.remaining.is_zero() {
            self.running = false;
            self.last_tick = None;
        } else {
            self.last_tick = Some(Instant::now());
        }
    }

    fn progress(&self) -> f32 {
        let total = self.mode.duration().as_secs_f32();
        1.0 - (self.remaining.as_secs_f32() / total)
    }
}

impl eframe::App for PomodoroApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tick();
        if self.running {
            ctx.request_repaint();
        }

        // Themed background per mode
        let bg = self.mode.color();
        let mut style = (*ctx.style()).clone();
        style.visuals.panel_fill = bg;
        style.visuals.window_fill = bg;
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);

                // Mode selector tabs
                ui.horizontal(|ui| {
                    for mode in [Mode::Work, Mode::ShortBreak, Mode::LongBreak] {
                        let selected = self.mode == mode;
                        let text_alpha: u8 = if selected { 255 } else { 150 };
                        let fill = if selected {
                            Color32::from_rgba_unmultiplied(255, 255, 255, 30)
                        } else {
                            Color32::TRANSPARENT
                        };
                        let btn = egui::Button::new(
                            RichText::new(mode.label())
                                .size(13.0)
                                .color(Color32::from_rgba_unmultiplied(255, 255, 255, text_alpha)),
                        )
                        .fill(fill);
                        if ui.add(btn).clicked() {
                            self.switch_mode(mode);
                        }
                    }
                });

                ui.add_space(50.0);

                // Countdown display
                let secs = self.remaining.as_secs();
                ui.label(
                    RichText::new(format!("{:02}:{:02}", secs / 60, secs % 60))
                        .size(88.0)
                        .monospace()
                        .color(Color32::WHITE),
                );

                ui.add_space(16.0);

                // Progress bar
                ui.add(egui::ProgressBar::new(self.progress()).desired_width(240.0));

                ui.add_space(40.0);

                // Start / Pause
                let btn_label = if self.running { "  Pause  " } else { "  Start  " };
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new(btn_label).size(20.0).color(bg),
                        )
                        .fill(Color32::WHITE)
                        .min_size(egui::vec2(150.0, 50.0)),
                    )
                    .clicked()
                {
                    self.toggle();
                }

                ui.add_space(12.0);

                // Reset
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new("Reset")
                                .size(14.0)
                                .color(Color32::from_rgba_unmultiplied(255, 255, 255, 180)),
                        )
                        .fill(Color32::TRANSPARENT),
                    )
                    .clicked()
                {
                    self.reset();
                }
            });
        });
    }
}
