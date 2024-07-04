use crate::{credentials::Credentials, data_loader::DataLoader, Page};

use super::RegionsPage;

pub struct CredentialsPage {
    profiles: DataLoader<Vec<String>>,
    selected_profile: usize,
}

impl CredentialsPage {
    pub fn new() -> Self {
        Self {
            profiles: DataLoader::new(),
            selected_profile: 0,
        }
    }

    pub fn render_page(
        &mut self,
        ctx: &eframe::egui::Context,
        _: &eframe::Frame,
        rt: &tokio::runtime::Runtime,
    ) -> Option<Page> {
        let mut optional_page: Option<Page> = None;

        if !self.profiles.load_requested() {
            let sender = self.profiles.sender_clone();
            rt.spawn(async move {
                Self::load_profiles(sender).await;
            });
            self.profiles.mark_load_requested();
        }

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.heading("Credentials");

                egui::CollapsingHeader::new("Default")
                    .show(ui, |ui| {
                        if ui.button("Use default config").clicked() {
                            let regions_page = RegionsPage::new(Credentials::Default);
                            optional_page = Some(Page::Region(regions_page));
                        };
                    })
                    .body_returned;

                egui::CollapsingHeader::new("Profile")
                    .show(ui, |ui| {
                        self.profiles.poll();
                        if let Some(profiles) = self.profiles.data() {
                            egui::ComboBox::from_label("Choose profile")
                                .selected_text(profiles.get(self.selected_profile).unwrap())
                                .show_ui(ui, |ui| {
                                    for i in 0..profiles.len() {
                                        let profile = profiles.get(i).unwrap();
                                        ui.selectable_value(&mut self.selected_profile, i, profile);
                                    }
                                });

                            if ui.button("Use selected profile").clicked() {
                                let profile = profiles.get(self.selected_profile).unwrap();
                                let creds = Credentials::Profile(profile.to_string());
                                let regions_page = RegionsPage::new(creds);
                                optional_page = Some(Page::Region(regions_page));
                            }
                        }
                    })
                    .body_returned;
            })
            .inner;

        optional_page
    }

    pub async fn load_profiles(sender: tokio::sync::mpsc::Sender<Vec<String>>) {
        let fs = aws_types::os_shim_internal::Fs::real();
        let env = aws_types::os_shim_internal::Env::real();
        let profile_files = aws_runtime::env_config::file::EnvConfigFiles::default();
        let env_config_sections = &aws_config::profile::load(&fs, &env, &profile_files, None)
            .await
            .unwrap();
        let profiles = env_config_sections.profiles().map(|p| String::from(p));
        let _ = sender.send(Vec::from_iter(profiles)).await;
    }
}
