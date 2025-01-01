use bollard::{container::ListContainersOptions, image::ListImagesOptions, Docker};

use crate::CollectAsyncStats;

use super::DockerStats;

#[derive(Debug)]
pub struct DockerMonitor {
    pub monitor: Docker,
}

impl DockerMonitor {
    pub fn new() -> Self {
        Self {
            monitor: Docker::connect_with_local_defaults().unwrap(),
        }
    }
}

impl CollectAsyncStats for DockerMonitor {
    type StatsType = DockerStats;

    async fn collect_stats(&mut self) -> Result<DockerStats, String> {
        let list_image_options = Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        });
        let images = self
            .monitor
            .list_images(list_image_options)
            .await
            .or_else(|_| Err("could not list docker images".to_string()))?;

        let containers = self
            .monitor
            .list_containers(Some(ListContainersOptions::<String> {
                all: true,
                ..Default::default()
            }))
            .await
            .or_else(|_| Err("could not list docker containers".to_string()))?;

        Ok(DockerStats { images, containers })
    }
}
