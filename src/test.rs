use crate::ast::{
    DomainOrIp, HttpsSpec, ListenAddress, ListenAddressWithProtocol, OperationMode, Protocol,
};
use crate::tailscale_serve_status::HttpsSpecListParser;

#[test]
fn test_https_parser() {
    let parser = HttpsSpecListParser::new();
    let text = "

https://f2koi-on-ts-net.puella-magi-madoka-magica.ts.net:32345 (tailnet only)
|-- / proxy http://127.0.0.1:34342

https://f2koi-on-ts-net.puella-magi-madoka-magica.ts.net:41414 (Funnel on)
|-- / proxy http://127.0.0.1:41415

https://f2koi-on-ts-net.puella-magi-madoka-magica.ts.net (tailnet only)
|-- / proxy http://127.0.0.1:3000

";
    let expected_result = vec![
        HttpsSpec {
            mode: OperationMode::Serve,
            tailscale_listen_addresses: vec![ListenAddressWithProtocol {
                protocol: Protocol::Https,
                listen_address: ListenAddress {
                    domain_or_ip: DomainOrIp::Domain {
                        domain_name: "f2koi-on-ts-net.puella-magi-madoka-magica.ts.net".to_string(),
                    },
                    port: 32345,
                },
            }],
            proxy_target_port: 34342,
        },
        HttpsSpec {
            mode: OperationMode::Funnel,
            tailscale_listen_addresses: vec![ListenAddressWithProtocol {
                protocol: Protocol::Https,
                listen_address: ListenAddress {
                    domain_or_ip: DomainOrIp::Domain {
                        domain_name: "f2koi-on-ts-net.puella-magi-madoka-magica.ts.net".to_string(),
                    },
                    port: 41414,
                },
            }],
            proxy_target_port: 41415,
        },
        HttpsSpec {
            mode: OperationMode::Serve,
            tailscale_listen_addresses: vec![ListenAddressWithProtocol {
                protocol: Protocol::Https,
                listen_address: ListenAddress {
                    domain_or_ip: DomainOrIp::Domain {
                        domain_name: "f2koi-on-ts-net.puella-magi-madoka-magica.ts.net".to_string(),
                    },
                    port: 443,
                },
            }],
            proxy_target_port: 3000,
        },
    ];

    let parse_result = parser.parse(text).unwrap();

    for (expected, actual) in expected_result.into_iter().zip(parse_result) {
        assert_eq!(expected, actual);
    }
}
