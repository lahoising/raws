mod alarms;
mod home;

use home::CloudWatchHomePage;
use tokio::runtime::Runtime;

use crate::aws_clients::AwsClients;

use self::alarms::AlarmsPage;

use super::ServicePageType;

const TABS: &'static [(&'static str, fn(&CloudWatchPage) -> CloudWatchPageType)] = &[
    ("Home", CloudWatchPage::home_page),
    ("Alarms", CloudWatchPage::alarms_page),
];

pub enum CloudWatchPageType {
    Home(CloudWatchHomePage),
    Alarms(AlarmsPage),
}

pub struct CloudWatchPage {
    selected_page: usize,
    page: CloudWatchPageType,
}

impl CloudWatchPage {
    pub fn new() -> Self {
        let home_page = CloudWatchPageType::Home(CloudWatchHomePage::new());
        let selected_page = TABS.iter().position(|&(page, _)| page == "Home").unwrap();
        Self {
            page: home_page,
            selected_page,
        }
    }

    pub fn render_page(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
        rt: &Runtime,
        aws_clients: &AwsClients,
    ) -> Option<ServicePageType> {
        egui::TopBottomPanel::bottom("cw page picker").show(ctx, |ui| {
            egui::ComboBox::from_label("")
                .selected_text(TABS[self.selected_page].0)
                .show_ui(ui, |ui| {
                    for i in 0..TABS.len() {
                        let tab = TABS[i];
                        let tab_name = tab.0;
                        if ui
                            .selectable_value(&mut self.selected_page, i, tab_name)
                            .clicked()
                        {
                            self.page = tab.1(self);
                        }
                    }
                });
        });

        match &mut self.page {
            CloudWatchPageType::Home(page) => page.render_page(ctx, frame),
            CloudWatchPageType::Alarms(page) => page.render_page(ctx, frame, rt, aws_clients),
        };
        None
    }

    fn home_page(&self) -> CloudWatchPageType {
        CloudWatchPageType::Home(CloudWatchHomePage::new())
    }

    fn alarms_page(&self) -> CloudWatchPageType {
        CloudWatchPageType::Alarms(AlarmsPage::new())
    }
}
