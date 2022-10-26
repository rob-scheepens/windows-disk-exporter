use clap::Parser;
use std::net::Ipv4Addr;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    #[arg(
        long,
        default_value = "127.0.0.1",
        help = "Address on which to expose metrics and web interface."
    )]
    pub ipaddr: Ipv4Addr,
    #[arg(
        long,
        default_value = "9184",
        help = "Port on which to expose metrics and web interface."
    )]
    pub port: u16,
}
