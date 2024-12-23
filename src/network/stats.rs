extern crate serde;

use self::serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct IpNetwork {
    pub ip: String,
    pub version: String,
    pub prefix: u8,
}

#[derive(Debug, Default, Serialize)]
pub struct NetworkInterface {
    pub name: String,
    pub received: u64,
    pub total_received: u64,
    pub transmitted: u64,
    pub total_transmitted: u64,
    pub packets_received: u64,
    pub total_packets_received: u64,
    pub packages_transmitted: u64,
    pub total_packages_transmitted: u64,
    pub errors_on_received: u64,
    pub total_errors_on_received: u64,
    pub errors_on_transmitted: u64,
    pub total_errors_on_transmitted: u64,
    pub mac_address: [u8; 6],
    pub ip_networks: Vec<IpNetwork>,
    pub mtu: u64,
}

#[derive(Debug, Serialize)]
pub struct NetworkStats(pub Vec<NetworkInterface>);

impl From<&sysinfo::NetworkData> for NetworkInterface {
    fn from(value: &sysinfo::NetworkData) -> Self {
        Self {
            name: String::default(),
            received: value.received(),
            total_received: value.total_received(),
            transmitted: value.transmitted(),
            total_transmitted: value.total_transmitted(),
            packets_received: value.packets_received(),
            total_packets_received: value.total_packets_received(),
            packages_transmitted: value.packets_transmitted(),
            total_packages_transmitted: value.total_packets_transmitted(),
            errors_on_received: value.errors_on_received(),
            total_errors_on_received: value.total_errors_on_received(),
            errors_on_transmitted: value.errors_on_transmitted(),
            total_errors_on_transmitted: value.total_errors_on_transmitted(),
            mac_address: value.mac_address().0,
            ip_networks: value
                .ip_networks()
                .iter()
                .map(|ipn| IpNetwork {
                    ip: ipn.addr.to_string(),
                    version: match ipn.addr {
                        std::net::IpAddr::V4(_) => String::from("v4"),
                        std::net::IpAddr::V6(_) => String::from("v6"),
                    },
                    prefix: ipn.prefix,
                })
                .collect(),
            mtu: value.mtu(),
        }
    }
}
