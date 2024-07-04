use aws_sdk_cloudwatch::{
    error::SdkError,
    operation::describe_alarms::{DescribeAlarmsError, DescribeAlarmsOutput},
    Client as CloudWatchAgent,
};
use tokio::runtime::Runtime;

use crate::{aws_clients::AwsClients, data_loader::DataLoader};

use super::CloudWatchPageType;

pub struct AlarmsPage {
    alarms: DataLoader<Result<DescribeAlarmsOutput, SdkError<DescribeAlarmsError>>>,
}

impl AlarmsPage {
    pub fn new() -> Self {
        Self {
            alarms: DataLoader::new(),
        }
    }

    pub fn render_page(
        &mut self,
        ctx: &eframe::egui::Context,
        _: &eframe::Frame,
        rt: &Runtime,
        aws_clients: &AwsClients,
    ) -> Option<CloudWatchPageType> {
        self.describe_alarms_if_needed(ctx.clone(), rt, aws_clients.cloudwatch().clone());

        egui::CentralPanel::default().show(ctx, |ui| {
            self.alarms.poll();
            match self.alarms.data() {
                Some(alarms_response) => match alarms_response {
                    Ok(alarms_data) => {
                        for item in alarms_data.metric_alarms() {
                            ui.horizontal(|ui| {
                                ui.label(item.alarm_name().unwrap());
                                ui.label(item.state_value().unwrap().to_string());
                            });
                        }
                    }
                    Err(err) => {
                        ui.label(
                            egui::RichText::new("Failed to describe alarms")
                                .color(egui::Color32::RED),
                        );
                        ui.label(egui::RichText::new(err.to_string()).color(egui::Color32::RED));
                    }
                },
                None => {
                    ui.label("Loading alarms...");
                }
            }
        });
        None
    }

    fn describe_alarms_if_needed(
        &mut self,
        ctx: eframe::egui::Context,
        rt: &Runtime,
        cw: CloudWatchAgent,
    ) {
        if self.alarms.load_requested() {
            return;
        }
        let alarms_sender = self.alarms.sender_clone();
        rt.spawn(async move {
            let response = cw.describe_alarms().send().await;
            let _ = alarms_sender.send(response).await;
            ctx.request_repaint();
        });
        self.alarms.mark_load_requested();
    }
}
