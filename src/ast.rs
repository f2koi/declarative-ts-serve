use std::net::Ipv4Addr;

#[derive(Debug, Eq, PartialEq)]
pub struct HttpSpec {
    pub tailscale_listen_addresses: Vec<ListenAddressWithProtocol>,
    pub proxy_target_port: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct HttpsSpec {
    pub mode: OperationMode,
    pub tailscale_listen_addresses: Vec<ListenAddressWithProtocol>,
    pub proxy_target_port: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ListenAddressWithProtocol {
    pub protocol: Protocol,
    pub listen_address: ListenAddress,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ListenAddress {
    pub domain_or_ip: DomainOrIp,
    pub port: u16,
}

#[derive(Debug, Eq, PartialEq)]
pub enum DomainOrIp {
    Domain { domain_name: String },
    Ipv4(Ipv4Addr),
    Ipv6, // Parsing IPv6 is toooooooooooooooooooooooooooo0ooOoo00Ooo hard
}

#[derive(Debug, Eq, PartialEq)]
pub enum OperationMode {
    Funnel,
    Serve,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Protocol {
    Http,
    Https,
    Tcp,
}

impl Protocol {
    pub fn default_port(&self) -> u16 {
        match self {
            Protocol::Http => 80,
            Protocol::Https => 443,
            _ => panic!("Partial function"),
        }
    }
}
