use async_trait::async_trait;
use bollard::image::ListImagesOptions;
use bollard::{container::ListContainersOptions, errors::Error, Docker};

use crate::docker::DockerStats;
use crate::Monitor;

#[derive(Debug)]
pub struct DockerMonitor {
    pub monitor: Docker,
}

impl DockerMonitor {
    pub fn new() -> Result<Self, Error> {
        let monitor = Docker::connect_with_local_defaults()?;
        Ok(Self { monitor })
    }
}

#[async_trait]
impl Monitor for DockerMonitor {
    async fn report(&mut self) -> serde_json::Value {
        let image_options = ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        };

        let container_options = ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        };

        let images = self.monitor.list_images(Some(image_options)).await.unwrap();
        let containers = self.monitor.list_containers(Some(container_options)).await.unwrap();
        let docker_stats = DockerStats { images, containers };


        serde_json::Value::try_from(docker_stats).unwrap()
    }
}
