use lalrpop_util::lalrpop_mod;

#[allow(dead_code)]
mod ast;
lalrpop_mod!(pub tailscale_serve_status);

fn main() {
    let parser = tailscale_serve_status::HttpsSpecParser::new();
    let text = "\
https://f2koi-windows.whale-dominant.ts.net:32345 (tailnet only)
|-- / proxy http://127.0.0.1:34342";

    let spec = parser.parse(text).unwrap();
    println!("{:?}", spec);
}
