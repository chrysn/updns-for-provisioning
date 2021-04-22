//! A side tool usable as `cargo run --bin convert -- 2001:db8::1`

fn main() {
    let ip = std::env::args().nth(1).expect("Please give an IP address as an argument");

    let ip: std::net::Ipv6Addr = ip.parse().expect("Failed to parse");
    println!("{}.at.hex-key.example.com", data_encoding::BASE32_DNSSEC.encode(&ip.octets()));
}
