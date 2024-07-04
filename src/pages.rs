mod credentials;
mod regions;
mod service_pages;

pub use credentials::CredentialsPage;
pub use regions::RegionsPage;
pub use service_pages::ServicePage;

use crate::Config;

pub enum Page {
    Credentials(CredentialsPage),
    Region(RegionsPage),
    Service(ServicePage),
}

impl Page {
    pub fn render_page(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
        rt: &tokio::runtime::Runtime,
        config: &mut Config,
    ) -> Option<Page> {
        match self {
            Page::Credentials(page) => page.render_page(ctx, frame, rt),
            Page::Region(page) => page.render_page(ctx, frame, rt, config),
            Page::Service(page) => page.render_page(ctx, frame, rt),
        }
    }
}
