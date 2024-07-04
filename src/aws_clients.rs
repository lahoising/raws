use crate::{credentials::Credentials, regions::Region};
use aws_config::BehaviorVersion;
use aws_sdk_cloudwatch::Client as CloudWatchClient;

pub struct AwsClients {
    cloudwatch: CloudWatchClient,
}

impl AwsClients {
    pub async fn new(creds: &Credentials, region: &Region) -> Self {
        let owned_region: aws_config::Region = region.to_owned().into();
        let endpoint = format!("https://monitoring.{}.{}", region.name(), region.domain());
        let cloudwatch_config = creds
            .config_loader()
            .region(owned_region)
            .endpoint_url(endpoint)
            .behavior_version(BehaviorVersion::latest())
            .load()
            .await;

        Self {
            cloudwatch: CloudWatchClient::new(&cloudwatch_config),
        }
    }

    pub fn cloudwatch(&self) -> &CloudWatchClient {
        &self.cloudwatch
    }
}
