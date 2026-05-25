#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod app;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([340.0, 500.0])
            .with_min_inner_size([340.0, 500.0])
            .with_max_inner_size([340.0, 500.0])
            .with_resizable(false)
            .with_title("Pomodoro Timer"),
        ..Default::default()
    };
    eframe::run_native(
        "Pomodoro Timer",
        options,
        Box::new(|_cc| Ok(Box::new(app::PomodoroApp::new()))),
    )
}
