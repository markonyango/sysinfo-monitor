use bollard::{container::ListContainersOptions, image::ListImagesOptions, Docker};

use crate::{CollectAsyncStats, Monitor, MonitorData};

use super::DockerStats;

#[derive(Debug)]
pub struct DockerMonitor {
    pub monitor: Option<Docker>,
}

impl DockerMonitor {
    pub fn new() -> Self {
        Self {
            monitor: Docker::connect_with_local_defaults().ok(),
        }
    }
}

impl CollectAsyncStats for DockerMonitor {
    async fn collect_stats(&mut self) -> MonitorData {
        let list_image_options = Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        });

        let list_container_options = Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        });

        if let Some(monitor) = &self.monitor {
            let images = monitor.list_images(list_image_options).await.ok();
            let containers = monitor.list_containers(list_container_options).await.ok();

            return MonitorData::Docker(DockerStats { images, containers });
        }

        MonitorData::Docker(DockerStats {
            images: None,
            containers: None,
        })
    }
}

impl Monitor for DockerMonitor {
    fn report(&mut self) -> serde_json::Value {
        todo!()
    }
}
