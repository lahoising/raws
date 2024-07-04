mod aws_clients;
mod config;
mod credentials;
mod data_loader;
mod pages;
mod regions;

use config::Config;
use eframe::egui;
use pages::{CredentialsPage, Page};

fn main() {
    let options = eframe::NativeOptions::default();
    let credentials_page = CredentialsPage::new();
    let _ = eframe::run_native(
        "RAWS",
        options,
        Box::new(|cc| Box::new(Raws::new(cc, Page::Credentials(credentials_page)))),
    );
}

struct Raws {
    current_page: Page,
    config: Config,
    rt: tokio::runtime::Runtime,
}

impl Raws {
    fn new(_: &eframe::CreationContext<'_>, initial_page: Page) -> Self {
        Self {
            current_page: initial_page,
            config: Config::load().unwrap_or(Config::default()),
            rt: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        }
    }
}

impl eframe::App for Raws {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self
            .current_page
            .render_page(ctx, frame, &self.rt, &mut self.config)
        {
            Some(new_page) => self.current_page = new_page,
            None => {}
        }
    }
}
