use egui::TextBuffer;

use super::{cloudwatch_pages::CloudWatchPage, ServicePageType};

const AVAILABLE_SERVICES: &'static [(&'static str, fn(&ServicesHomePage) -> ServicePageType)] = &[
    ("CloudWatch", ServicesHomePage::cloudwatch_page),
    // ("CloudFormation", ServicesHomePage::cloudformation_page),
    // ("DynamoDB", ServicesHomePage::dynamodb_page),
    // ("CodeDeploy", ServicesHomePage::codedeploy_page),
];

pub struct ServicesHomePage {
    service_search: String,
}

impl ServicesHomePage {
    pub fn new() -> Self {
        Self {
            service_search: String::from(""),
        }
    }

    pub fn render_page(
        &mut self,
        ctx: &eframe::egui::Context,
        _: &eframe::Frame,
    ) -> Option<ServicePageType> {
        let mut optional_service_page: Option<ServicePageType> = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Choose a service!");

            ui.text_edit_singleline(&mut self.service_search);

            for (service, page_loader) in AVAILABLE_SERVICES {
                if service
                    .to_lowercase()
                    .contains(&self.service_search.to_lowercase())
                {
                    if ui.button(service.as_str()).clicked() {
                        optional_service_page = Some(page_loader(self));
                    }
                }
            }
        });
        optional_service_page
    }

    pub fn cloudwatch_page(&self) -> ServicePageType {
        ServicePageType::CloudWatch(CloudWatchPage::new())
    }

    // pub fn cloudformation_page(&self) {
    //     println!("CloudFormation!");
    // }

    // pub fn codedeploy_page(&self) {
    //     println!("CodeDeploy!");
    // }

    // pub fn dynamodb_page(&self) {
    //     println!("DynamoDB!");
    // }
}
