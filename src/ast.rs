use std::net::SocketAddrV4;

#[derive(Debug)]
pub struct HttpSpec {
    pub tailscale_listen_addresses: Vec<ListenAddress>,
    pub proxy_target_port: u16,
}

#[derive(Debug)]
pub struct HttpsSpec {
    pub mode: OperationMode,
    pub tailscale_listen_addresses: Vec<ListenAddress>,
    pub proxy_target_port: u16,
}

#[derive(Debug)]
pub enum ListenAddress {
    Domain { domain_name: String, port: u16 },
    Ipv4(SocketAddrV4),
    Ipv6 { port: u16 }, // Parsing IPv6 is toooooooooooooooooooooooooooo0ooOoo00Ooo hard
}

#[derive(Debug)]
pub enum OperationMode {
    Funnel,
    Serve,
}
