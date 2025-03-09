use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct HttpSpec {
    pub tailscale_listen_addresses: Vec<ListenAddressWithProtocol>,
    pub proxy_target_port: u16,
}

#[derive(Debug)]
pub struct HttpsSpec {
    pub mode: OperationMode,
    pub tailscale_listen_addresses: Vec<ListenAddressWithProtocol>,
    pub proxy_target_port: u16,
}

#[derive(Debug)]
pub struct ListenAddressWithProtocol {
    pub protocol: Protocol,
    pub listen_address: ListenAddress,
}

#[derive(Debug)]
pub struct ListenAddress {
    pub domain_or_ip: DomainOrIp,
    pub port: u16,
}

#[derive(Debug)]
pub enum DomainOrIp {
    Domain { domain_name: String },
    Ipv4(Ipv4Addr),
    Ipv6, // Parsing IPv6 is toooooooooooooooooooooooooooo0ooOoo00Ooo hard
}

#[derive(Debug)]
pub enum OperationMode {
    Funnel,
    Serve,
}

#[derive(Debug)]
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
