mod cloudwatch_pages;
mod services_home;

use crate::{
    aws_clients::AwsClients,
    credentials::Credentials,
    data_loader::DataLoader,
    pages::{
        service_pages::{cloudwatch_pages::CloudWatchPage, services_home::ServicesHomePage},
        Page,
    },
    regions::Region,
};
use tokio::runtime::Runtime;

pub struct ServicePage {
    aws_clients: DataLoader<AwsClients>,
    page: ServicePageType,
}

pub enum ServicePageType {
    Home(ServicesHomePage),
    CloudWatch(CloudWatchPage),
}

impl ServicePage {
    pub fn home_page(creds: Credentials, region: Region, rt: &tokio::runtime::Runtime) -> Self {
        let aws_clients = ServicePage::create_aws_clients_data_loader(creds, region, rt);
        let home_page = ServicesHomePage::new();
        Self {
            aws_clients,
            page: ServicePageType::Home(home_page),
        }
    }

    fn create_aws_clients_data_loader(
        creds: Credentials,
        region: Region,
        rt: &tokio::runtime::Runtime,
    ) -> DataLoader<AwsClients> {
        let mut aws_clients = DataLoader::new();

        if !aws_clients.load_requested() {
            let sender = aws_clients.sender_clone();
            rt.spawn(async move {
                let aws_clients = AwsClients::new(&creds, &region).await;
                let _ = sender.send(aws_clients).await;
            });
            aws_clients.mark_load_requested();
        }

        aws_clients
    }

    pub fn render_page(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
        rt: &Runtime,
    ) -> Option<Page> {
        self.aws_clients.poll();
        if self.aws_clients.data().is_none() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Initializing AWS clients...");
            });
            return None;
        }

        let optional_service_page = match &mut self.page {
            ServicePageType::Home(page) => page.render_page(ctx, frame),
            ServicePageType::CloudWatch(page) => {
                page.render_page(ctx, frame, rt, self.aws_clients.data().as_ref().unwrap())
            }
        };

        if let Some(page) = optional_service_page {
            self.page = page;
        }

        None
    }
}
