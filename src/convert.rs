//! A side tool usable as `cargo run --bin convert -- 2001:db8::1`

fn main() {
    let ip = std::env::args().nth(1).expect("Please give an IP address as an argument");

    let ip: std::net::IpAddr = ip.parse().expect("Failed to parse");
    let octets = match ip {
        std::net::IpAddr::V4(i) => (&i.octets()[..]).to_owned(),
        std::net::IpAddr::V6(i) => (&i.octets()[..]).to_owned(),
    };
    println!("at-{}.at.hex-key.example.com", data_encoding::BASE32_DNSSEC.encode(&octets));
}
