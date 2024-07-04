use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Region {
    name: String,
    domain: String,
}

impl Region {
    pub fn default() -> Self {
        Self::new("us-east-1", "amazonaws.com")
    }

    pub fn new<S: Into<String>>(name: S, domain: S) -> Self {
        Self {
            name: name.into(),
            domain: domain.into(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn domain(&self) -> &String {
        &self.domain
    }
}

impl Into<aws_config::Region> for Region {
    fn into(self) -> aws_config::Region {
        aws_config::Region::new(self.name)
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = format!("[{}: {}]", self.name, self.domain);
        f.write_str(&display)
    }
}
