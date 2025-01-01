use bollard::secret::{ContainerSummary, ImageSummary};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct DockerStats {
    pub images: Vec<ImageSummary>,
    pub containers: Vec<ContainerSummary>,
}
