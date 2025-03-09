use lalrpop_util::lalrpop_mod;

#[allow(dead_code)]
mod ast;
lalrpop_mod!(pub tailscale_serve_status);

#[cfg(test)]
mod test;

fn main() {}
