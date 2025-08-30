use bollard::secret::{ContainerSummary, ImageSummary};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DockerStats {
    pub images: Vec<ImageSummary>,
    pub containers: Vec<ContainerSummary>,
}

impl TryFrom<DockerStats> for serde_json::Value {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: DockerStats) -> Result<Self, Self::Error> {
       serde_json::to_value(value).map_err(Into::into)
    }

}
