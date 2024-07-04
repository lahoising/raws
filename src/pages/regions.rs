use crate::{config::Config, credentials::Credentials, pages::ServicePage, regions::Region, Page};
use eframe::egui;

const SECTION_SPACING: f32 = 15.0;

pub struct RegionsPage {
    creds: Credentials,
    selected_region: String,
    new_region_name: String,
    new_region_domain: String,
}

impl RegionsPage {
    pub fn new(creds: Credentials) -> Self {
        Self {
            creds,
            selected_region: String::from(""),
            new_region_name: String::from(""),
            new_region_domain: String::from(""),
        }
    }

    pub fn render_page(
        &mut self,
        ctx: &eframe::egui::Context,
        _: &eframe::Frame,
        rt: &tokio::runtime::Runtime,
        config: &mut Config,
    ) -> Option<Page> {
        let mut optional_page: Option<Page> = None;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Region");

            ui.horizontal(|ui| {
                if ui.button("Use region").clicked() {
                    let optional_region = config.regions().get(&self.selected_region);
                    if let Some(region) = optional_region {
                        let services_home_page =
                            ServicePage::home_page(self.creds.clone(), region.clone(), rt);
                        optional_page = Some(Page::Service(services_home_page));
                    }
                }

                let all_regions = config.regions().keys();
                if all_regions.len() > 0 {
                    egui::ComboBox::from_label("Choose a region")
                        .selected_text(&self.selected_region)
                        .show_ui(ui, |ui| {
                            all_regions.for_each(|region| {
                                ui.selectable_value(
                                    &mut self.selected_region,
                                    region.into(),
                                    region,
                                );
                            });
                        });
                }
            });

            ui.add_space(SECTION_SPACING);
            ui.heading("Add new regions");
            ui.label("If you want to specify new custom regions, add them below");
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.new_region_name);
            });
            ui.horizontal(|ui| {
                ui.label("Domain:");
                ui.text_edit_singleline(&mut self.new_region_domain);
            });
            if ui.button("Submit new region").clicked() {
                let new_region = Region::new(&self.new_region_name, &self.new_region_domain);
                let set_region_result = config.set_region(new_region);
                if set_region_result.is_err() {
                    eprintln!(
                        "Error while trying to set region: {:#?}",
                        set_region_result.err().unwrap()
                    );
                }
            }
        });
        optional_page
    }
}
