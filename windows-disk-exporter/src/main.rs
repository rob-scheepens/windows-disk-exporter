mod cli;
mod collector;

use clap::Parser;
// use prometheus_exporter::prometheus::register_counter;
use std::net::IpAddr;
use std::net::SocketAddr;
use windows::{core::*, Win32::System::Performance::*};

fn main() {
    let args = cli::Args::parse();
    let binding = SocketAddr::new(IpAddr::V4(args.ipaddr), args.port);
    // Will create an exporter and start the http server using the given binding.
    // If the webserver can't bind to the given binding it will fail with an error.
    prometheus_exporter::start(binding).unwrap();

    // Create a counter using the global prometheus registry and increment it by one.
    // Notice that the macro is coming from the reexported prometheus crate instead
    // of the original crate. This is important as different versions of the
    // prometheus crate have incompatible global registries.
    // let counter = register_counter!("user_exporter_counter", "help").unwrap();
    //	counter.inc();
    unsafe {
        // Create Performance Query
        let mut query = 0;
        PdhOpenQueryW(None, 0, &mut query);

        let mut disksecwrite = 0;
        PdhAddCounterW(
            query,
            w!("\\physicaldisk(1)\\avg. disk sec/write"),
            0,
            &mut disksecwrite,
        );

        let mut disksecread = 0;
        PdhAddCounterW(
            query,
            w!("\\physicaldisk(1)\\avg. disk sec/read"),
            0,
            &mut disksecread,
        );
        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
            PdhCollectQueryData(query);

            let mut value_write = Default::default();
            if 0 == PdhGetFormattedCounterValue(
                disksecwrite,
                PDH_FMT_DOUBLE,
                None,
                &mut value_write,
            ) {
                let mut value_read = Default::default();
                if 0 == PdhGetFormattedCounterValue(
                    disksecread,
                    PDH_FMT_DOUBLE,
                    None,
                    &mut value_read,
                ) {
                    println!(
                        "{:.4}\t  {:.4}",
                        value_write.Anonymous.doubleValue * 1000.0,
                        value_read.Anonymous.doubleValue * 1000.0
                    );
                }
            }
        }
    }
}
