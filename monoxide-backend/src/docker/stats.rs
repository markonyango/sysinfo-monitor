use bollard::secret::{ContainerSummary, ImageSummary};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct DockerStats {
    pub images: Option<Vec<ImageSummary>>,
    pub containers: Option<Vec<ContainerSummary>>,
}
