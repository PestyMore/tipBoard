#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod types;
mod storage;
mod ui;
mod config;
mod app;

use app::TipBoardApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // 加载任务栏图标
    let icon_data = include_bytes!("../icon.png");
    let icon = eframe::icon_data::from_png_bytes(icon_data).expect("Failed to load icon");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false) 
            .with_transparent(true)  
            .with_always_on_top()    
            .with_icon(icon)
            .with_inner_size([config::WINDOW_WIDTH, config::WINDOW_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native("tipBoard", options, Box::new(|cc| Ok(Box::new(TipBoardApp::new(cc)))))
}
