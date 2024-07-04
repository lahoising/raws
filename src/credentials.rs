use aws_config::ConfigLoader;

#[derive(Clone)]
pub enum Credentials {
    Default,
    Profile(String),
}

impl Credentials {
    pub fn config_loader(&self) -> ConfigLoader {
        match self {
            Credentials::Default => ConfigLoader::default(),
            Credentials::Profile(profile) => ConfigLoader::default().profile_name(profile),
        }
    }
}
