use super::CloudWatchPageType;

pub struct CloudWatchHomePage {}

impl CloudWatchHomePage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_page(
        &mut self,
        _: &eframe::egui::Context,
        _: &eframe::Frame,
    ) -> Option<CloudWatchPageType> {
        None
    }
}
